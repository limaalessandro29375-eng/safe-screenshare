package xyz.bl4ckzx.safescreenshare.commands;

import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;
import org.bukkit.ChatColor;

public class ScreenShareCommand implements CommandExecutor {

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        if (!(sender instanceof Player)) {
            sender.sendMessage(ChatColor.RED + "Este comando deve ser usado por um jogador.");
            return true;
        }

        if (args.length == 0) {
            sender.sendMessage(ChatColor.RED + "Uso: /ss <jogador>");
            return true;
        }

        Player target = sender.getServer().getPlayer(args[0]);
        if (target == null) {
            sender.sendMessage(ChatColor.RED + "Jogador não encontrado.");
            return true;
        }

        // Lógica de telagem será implementada aqui
        sender.sendMessage(ChatColor.GREEN + "Iniciando telagem de " + target.getName() + "...");
        
        return true;
    }
}
