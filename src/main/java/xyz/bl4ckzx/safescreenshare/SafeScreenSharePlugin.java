package xyz.bl4ckzx.safescreenshare;

import org.bukkit.plugin.java.JavaPlugin;
import xyz.bl4ckzx.safescreenshare.commands.AutoScreenShareCommand;
import xyz.bl4ckzx.safescreenshare.commands.ConsentCommand;
import xyz.bl4ckzx.safescreenshare.commands.ScreenShareCommand;

public class SafeScreenSharePlugin extends JavaPlugin {

    @Override
    public void onEnable() {
        getLogger().info("SafeScreenShare habilitado com sucesso!");
        AutoScreenShareCommand autoCommand = new AutoScreenShareCommand();
        getCommand("ss").setExecutor(new ScreenShareCommand());
        getCommand("telagemautomatica").setExecutor(autoCommand);
        getCommand("aceitar").setExecutor(new ConsentCommand(autoCommand));
        getCommand("recusar").setExecutor(new ConsentCommand(autoCommand));
    }

    @Override
    public void onDisable() {
        getLogger().info("SafeScreenShare desabilitado.");
    }
}
