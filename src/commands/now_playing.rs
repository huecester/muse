use anyhow::anyhow;
use poise::command;

use crate::{format::now_playing_message, types::*};

/// View the currently playing song.
#[command(slash_command, guild_only, rename = "nowplaying")]
pub(crate) async fn now_playing(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let Some(manager) = songbird::get(ctx.serenity_context()).await else {
        return Err(anyhow!(SONGBIRD_MANAGER_ERR));
    };

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.send(|m| m.content("I'm not in a voice channel.").ephemeral(true)).await?;
        return Ok(());
    };

    let np = {
        let handler = handler_lock.lock().await;
        handler.queue().current()
    };

    let Some(np) = np else {
        ctx.send(|m| m.content("I'm not playing a song.").ephemeral(true)).await?;
        return Ok(());
    };

    ctx.send(|m| now_playing_message(m, np.metadata())).await?;
    Ok(())
}
