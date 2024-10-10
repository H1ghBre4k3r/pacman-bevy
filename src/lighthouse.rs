use std::{sync::Arc, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use lighthouse_client::{
    protocol::{Authentication, Frame},
    Lighthouse, TokioWebSocket,
};
use tokio::{runtime::Runtime, sync::Mutex};

#[derive(Debug, Clone)]
pub struct LighthousePlugin {
    pub user: String,
    pub token: String,
}

#[derive(Resource, Clone)]
struct LHWrapper(Arc<Mutex<Lighthouse<TokioWebSocket>>>);

#[derive(Resource)]
struct Rt(pub Runtime);

impl Plugin for LighthousePlugin {
    fn build(&self, app: &mut App) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let auth = Authentication::new(&self.user, &self.token);
            let Ok(lighthouse) =
                Lighthouse::connect_with_tokio_to("wss://lighthouse.uni-kiel.de/websocket", auth)
                    .await
            else {
                error!("can not connect to lighthouse");
                return;
            };

            app.insert_resource(LHWrapper(Arc::new(Mutex::new(lighthouse))));
            app.add_systems(
                PostUpdate,
                render_components.run_if(on_timer(Duration::from_secs_f64(1.0 / 60.0))),
            );
        });
        app.insert_resource(Rt(rt));
    }
}

fn render_components(
    rt: Res<Rt>,
    lh: Res<LHWrapper>,
    query: Query<(&LighthousePosition, &LighthouseColor)>,
) {
    let mut frame = Frame::empty();

    let mut entities = query.iter().collect::<Vec<_>>();
    entities.sort_by(|(a, _), (b, _)| a.z.cmp(&b.z));

    for (LighthousePosition { x, y, .. }, color) in entities {
        let y = 13 - y.min(&13);
        let x = x.min(&13);
        frame.set(x * 2, y, color.to_lighthouse());
        frame.set(x * 2 + 1, y, color.to_lighthouse());
    }

    let lh = lh.0.clone();

    rt.0.block_on(async move {
        if let Err(e) = lh.lock().await.put_model(frame).await {
            error!("Error sending to lighthouse: {e}");
        }
    });
}

#[derive(Debug, Clone, Bundle)]
pub struct LighthouseBundle {
    pub position: LighthousePosition,
    pub color: LighthouseColor,
}

#[derive(Debug, Clone, Component)]
pub struct LighthousePosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum LighthouseColor {
    Inline(u8, u8, u8),
    Explicit { r: u8, g: u8, b: u8 },
}

impl LighthouseColor {
    pub fn canonical(self) -> (u8, u8, u8) {
        match self {
            LighthouseColor::Inline(r, g, b) => (r, g, b),
            LighthouseColor::Explicit { r, g, b } => (r, g, b),
        }
    }

    pub fn to_lighthouse(self) -> lighthouse_client::protocol::Color {
        let (red, green, blue) = self.canonical();

        lighthouse_client::protocol::Color { red, green, blue }
    }
}
