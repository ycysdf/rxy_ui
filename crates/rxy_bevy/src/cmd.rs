use bevy_app::{App, Plugin, Update};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::{Deferred, World};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Command, Resource, SystemBuffer, SystemMeta};

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct CmdReceiver(pub async_channel::Receiver<bevy_ecs::system::CommandQueue>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct CmdSender(pub async_channel::Sender<bevy_ecs::system::CommandQueue>);

pub struct CommandChannelPlugin;

impl Plugin for CommandChannelPlugin {
    fn build(&self, app: &mut App) {
        let (cmd_sender, cmd_receiver) = async_channel::unbounded::<_>();

        app.insert_resource(CmdReceiver(cmd_receiver))
            .insert_resource(CmdSender(cmd_sender))
            .add_systems(Update, (recv_cmds.run_if(has_cmd),));
    }
}

impl CmdSender {
    pub fn add<C: Command>(&self, cmd: C) {
        let mut command_queue = bevy_ecs::system::CommandQueue::default();
        command_queue.push(cmd);
        self.send_blocking(command_queue).unwrap();
    }
}

#[derive(Resource, Default)]
pub struct CommandQueues {
    pub queues: Vec<bevy_ecs::system::CommandQueue>,
}

impl SystemBuffer for CommandQueues {
    fn apply(&mut self, _system_meta: &SystemMeta, world: &mut World) {
        for mut queue in self.queues.drain(..) {
            queue.apply(world);
        }
    }
}

pub fn recv_cmds(vdom_receiver: Res<CmdReceiver>, mut commands: Deferred<CommandQueues>) {
    while let Ok(command_queue) = vdom_receiver.try_recv() {
        commands.queues.push(command_queue);
    }
}

pub fn has_cmd(vdom_receiver: Res<CmdReceiver>) -> bool {
    !vdom_receiver.is_empty()
}
