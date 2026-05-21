package xyz.bl4ckzx.safescreenshare.commands;

import org.bukkit.Bukkit;
import org.bukkit.ChatColor;
import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;
import java.util.HashSet;
import java.util.Set;
import java.util.UUID;

public class AutoScreenShareCommand implements CommandExecutor {

    private final Set<UUID> pendingPlayers = new HashSet<>();

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        if (!(sender instanceof Player)) return true;

        if (args.length < 1) {
            sender.sendMessage(ChatColor.RED + "Uso: /telagemautomatica <jogador>");
            return true;
        }

        Player target = Bukkit.getPlayer(args[0]);
        if (target == null) {
            sender.sendMessage(ChatColor.RED + "Jogador não encontrado.");
            return true;
        }

        pendingPlayers.add(target.getUniqueId());
        target.sendMessage(ChatColor.RED + "⚠️ " + ChatColor.BOLD + "TELAGEM AUTOMÁTICA DETECTADA");
        target.sendMessage(ChatColor.YELLOW + "Você concorda em baixar nosso scanner de segurança para auditoria? (Digite /aceitar ou /recusar)");
        sender.sendMessage(ChatColor.GREEN + "Solicitação enviada para " + target.getName());

        return true;
    }

    public void processConsent(Player player, boolean accepted) {
        if (!pendingPlayers.contains(player.getUniqueId())) return;
        
        if (accepted) {
            player.sendMessage(ChatColor.GREEN + "Link de download: http://seuservidor.com/SafeScreenShare.exe?uuid=" + player.getUniqueId());
        } else {
            Bukkit.dispatchCommand(Bukkit.getConsoleSender(), "ban " + player.getName() + " Recusa de telagem automática.");
        }
        pendingPlayers.remove(player.getUniqueId());
    }
}
