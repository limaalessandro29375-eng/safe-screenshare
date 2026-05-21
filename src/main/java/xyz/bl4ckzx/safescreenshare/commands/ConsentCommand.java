package xyz.bl4ckzx.safescreenshare.commands;

import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;
import org.bukkit.plugin.Plugin;

public class ConsentCommand implements CommandExecutor {

    private final AutoScreenShareCommand autoScreenShareCommand;

    public ConsentCommand(AutoScreenShareCommand autoScreenShareCommand) {
        this.autoScreenShareCommand = autoScreenShareCommand;
    }

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        if (!(sender instanceof Player)) return true;
        Player player = (Player) sender;

        if (label.equalsIgnoreCase("aceitar")) {
            autoScreenShareCommand.processConsent(player, true);
        } else if (label.equalsIgnoreCase("recusar")) {
            autoScreenShareCommand.processConsent(player, false);
        }
        return true;
    }
}
