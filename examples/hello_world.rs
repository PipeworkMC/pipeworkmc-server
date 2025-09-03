use pipeworkmc_server::prelude::*;
use pipeworkmc_server::prelude::packet::*;
use packet::c2s::config::client_info::C2SConfigClientInfoPacket;
use core::time::Duration;
use bevy::{
    prelude::*,
    app::ScheduleRunnerPlugin,
};


fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(1)))
        )
        .add_systems(Update, status_response)
        .add_systems(Update, get_info)
        .run()
}


fn status_response(
    mut q_peers   : Query<(&mut ConnPeerSender,)>,
    mut er_status : EventReader<IncomingStatusPacketEvent>,
) {
    for e in er_status.read() {
        if let C2SStatusPackets::Request(_) = e.packet()
            && let Ok((mut sender,)) = q_peers.get_mut(e.peer())
        {
            sender.send(Status {
                motd    : Some(("Hello,".bold().red() + " World!").italic()),
                favicon : Some("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAMAAACdt4HsAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAABsUExURUdwTAAAAAAAAHdcTXNWSAAAAAAAAItqWzkmHQcEA5x3ZFY8LlQ4LHBRQWQ9L35hUEg4L2s6KnE/LmBLQFdEOoFjUotNOmdTR4BINk89M3lDMzwzLz49PJlVQG5HOXxRQatfSFo2KignJmNgXyXMkvwAAAARdFJOUwABCv7+Jhf+Yz3+mMrV58HrcyjiewAABM1JREFUWMPtV9lW6zgQhEi2bGchktpavSb//49TLScsl22YO4+IQwLEVaquXiQeHn7X7/puPW7rL6Av679Cd1g/YHgLreuqatumre8cj/+SYFewgO73h5Mxh31TVVW9+57hvu0NmrTRWhte+8NhX92C+Qq/A5ahJ81Yw994TfyT0U3btvVXDIx/mp25Q5NzbnQuGMcsemLa5otAGL8XISXejbFMgFdtFrzNEwV8cLg5+iG+rhoxBQBSwL4prCaEEQL0Ah3K0zSzrFPTVB8xAN8cHJuGpwrBmnOCiE3BLIWfx6CLIx8xlPgl6y37AzdRHGII4JymUU2zJFoWfIqUQET9J8Pjrm6eehemieEJAqaeLM3MJCh1YpYeBM44qNB0bv9g4ABOIz6apjk47BvcpHpxvVjnQGR72c+zXpagw8ge0+FtFCUAMTqEKGfZW/gVgur8NWeaO2+tlF4GmJmQ2wVGEb0NAvhG9Y49nOZeeQ/1QXrr8zCQtGTBYS2RTdqNy/hOAgfgIxVxBgSi79WGiszgfc+/IBJhkKBlSVqvryUggPpJEYgTJ9CpHgQKz3tvh2G4XPIgyEYrhTBpdMvIZtGhfZZQHOjnhXMYQkLOQcCRU74MmRnwlgdL0Yc0TtMELyFhX78Q1O2RpgUe6hSIWGvf9cLThXe/r0wZ3tgY12XcUnmTgN6onvB3B2JjZi8QLVklaXgG50u2ka7+eraeYuZEGEiotrZCBG0nKK6OiWc4KIW0V+L4c4Z8vOYsOA3Wyx5JcWyjjncJhaADsUGtmwn+CXG9XrJHEURAL8UFpMFbIYQSFlrd62oqIXR9ZGkgPh8EnkUtIAfFBY6EtpzI8tc0liEV4j2VXAZdh5xr1KJZI53P/upp8LLItiDIFomUqFFrz6dxKeMGdh82F8ok6XroTSwBaTh7Lj3ReY8WsiwhIi4BezgNU3E76Wj9GxcgYWUJGqWnelAglSDwhHKioe8Z38FfMc+zKUqtfCZ4hARVJPD4E6gkSV4wRolIfrgMLAYE4FEz5ptJCQKOt0QWCai9TQIaSnL0ECFRUTJniwogYgPQ4lIGntHJRDq9TAWeJyWVAfElU5zoEQB35OCtul4Hq7ifuGN0mfkoilcdyRKeOklcpoEHuyHbs20SHnqFfJJXHdfxuA3dkKMU91J8eEklRcxxfGEH62EFDwayHSqjtIfaZmIKweToxeuhUjqSJUxOh7WUpMV8NHhDBMgnhtRszHbUORboTzyc307FrvOYvVFnN+JYCNvxNj9Dy4mFhnc6R+y//2AsdkJxM2bEsGrYVU6Z7aQz3Opl4Du3ElIrm/eDuVVCyV5BBE6VdRx5OnPONihmjdkMIBhy5FPy/dnUHBGwwAhGG+JM0iEtZdewhb/yiqigvt9/cjo1I8yWSFykaV2zYR3YmKE8FRB8RK9IcXx/Nt0ZGiXkjJnAZlCMKZg3WMwUVEdT1Z+c0LigtM1xVAq9q7j/QQIaxmIgoJ14iX21+/SOwDO+GcPxFLhzuJxRS8/QDf/0Kf5GUeMu04x8PqCX1CusOB73+7b9Cn8TscOIwcMugOAFiktSVW9Xvu/vejtcEatmfxx7+Qq6+/FtsWXotu3uh7fe53vu7v+5Mv/dlf33357f9e36B75tiuIj3qHZAAAAAElFTkSuQmCC".into()),
                ..default()
            });
        }
    }
}

fn get_info(
    mut q_peers   : Query<(&AccountProfile, &mut ConnPeerSender,)>,
    mut er_config : EventReader<IncomingConfigPacketEvent>
) {
    for e in er_config.read() {
        if let C2SConfigPackets::ClientInfo(C2SConfigClientInfoPacket { info }) = e.packet()
            && let Ok((profile, mut sender,)) = q_peers.get_mut(e.peer())
        {
            sender.kick(
                profile.username.to_string().green().bold()
                + " " + ("(" + profile.uuid.to_string().yellow() + ")").grey()
                + "\n"
                + "\nLocale".white()             + ": ".grey() + info.locale.to_string().orange()
                + "\nView Distance".white()      + ": ".grey() + info.view_dist.to_string().green()
                + "\nChat Mode".white()          + ": ".grey() + format!("{:?}", info.chat_mode).red()
                + "\nChat Colours".white()       + ": ".grey() + info.chat_colours.to_string().cyan()
                + "\nSkin Layers".white()        + ": ".grey() + format!("{:0>7b}", info.skin_layers.as_byte() & 0b01111111).green()
                + "\nLeft Handed".white()        + ": ".grey() + info.left_handed.to_string().cyan()
                + "\nText Filtered".white()      + ": ".grey() + info.text_filtered.to_string().cyan()
                + "\nAllow MOTD Listing".white() + ": ".grey() + info.allow_motd_listing.to_string().cyan()
                + "\nParticle Status".white()    + ": ".grey() + format!("{:?}", info.particle_status).red()
            );
        }
    }
}
