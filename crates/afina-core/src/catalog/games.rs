//! Category "Jogos": settings that directly affect games.

use afina_platform::registry::Hive::{Hkcu, Hklm};

use crate::tweak::Val::{Dword, Sz};
use crate::tweak::{Category, Kind, Post, RegOp, Risk, Tweak};

const GAMEBAR: &str = r"Software\Microsoft\GameBar";
const MOUSE: &str = r"Control Panel\Mouse";

pub(super) static TWEAKS: &[Tweak] = &[
    Tweak {
        id: "game_mode",
        category: Category::Jogos,
        title: "Modo de Jogo",
        summary: "O Windows prioriza o jogo aberto e segura atualizações e notificações durante a partida.",
        details: "Configuração oficial do Windows (Configurações > Jogos > Modo de Jogo). Vem ligada de fábrica, mas muitos \"otimizadores\" desligam por engano.",
        risk: Risk::Seguro,
        admin: false,
        reboot: false,
        recommended: true,
        kind: Kind::Registry(&[
            RegOp::new(Hkcu, GAMEBAR, "AutoGameModeEnabled", Dword(1), Dword(0)).missing_means_on(),
            RegOp::new(Hkcu, GAMEBAR, "AllowAutoGameMode", Dword(1), Dword(0)).missing_means_on(),
        ]),
        post: Post::None,
    },
    Tweak {
        id: "game_dvr_off",
        category: Category::Jogos,
        title: "Desligar gravação em segundo plano (Xbox Game Bar)",
        summary: "Para de gravar a tela continuamente enquanto você joga, liberando GPU e disco.",
        details: "Desliga o Game DVR / captura de jogos. Você perde o atalho Win+Alt+R para gravar clipes pelo Windows (OBS, ShadowPlay e outros continuam funcionando).",
        risk: Risk::Seguro,
        admin: false,
        reboot: false,
        recommended: true,
        kind: Kind::Registry(&[
            RegOp::new(Hkcu, r"System\GameConfigStore", "GameDVR_Enabled", Dword(0), Dword(1)),
            RegOp::new(
                Hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                "AppCaptureEnabled",
                Dword(0),
                Dword(1),
            ),
        ]),
        post: Post::None,
    },
    Tweak {
        id: "windowed_opt",
        category: Category::Jogos,
        title: "Otimizações para jogos em janela",
        summary: "Menos atraso de imagem em jogos no modo \"janela sem bordas\" (DirectX 10/11).",
        details: "Mesma opção de Configurações > Sistema > Tela > Gráficos > Configurações padrão. Usa o modelo de apresentação moderno do DirectX, que também habilita VRR/Auto HDR em janela.",
        risk: Risk::Seguro,
        admin: false,
        reboot: false,
        recommended: true,
        kind: Kind::DxGlobalFlag { key: "SwapEffectUpgradeEnable" },
        post: Post::None,
    },
    Tweak {
        id: "mouse_accel_off",
        category: Category::Jogos,
        title: "Desligar aceleração do mouse",
        summary: "O ponteiro anda sempre a mesma distância para o mesmo movimento: mira mais consistente.",
        details: "Desmarca \"Aumentar a precisão do ponteiro\". A maioria dos jogos de tiro usa entrada bruta (raw input), mas menus, jogos antigos e a área de trabalho usam esta configuração.",
        risk: Risk::Seguro,
        admin: false,
        reboot: false,
        recommended: true,
        kind: Kind::Registry(&[
            RegOp::new(Hkcu, MOUSE, "MouseSpeed", Sz("0"), Sz("1")),
            RegOp::new(Hkcu, MOUSE, "MouseThreshold1", Sz("0"), Sz("6")),
            RegOp::new(Hkcu, MOUSE, "MouseThreshold2", Sz("0"), Sz("10")),
        ]),
        post: Post::Mouse,
    },
    Tweak {
        id: "hags",
        category: Category::Jogos,
        title: "Agendamento de GPU acelerado por hardware",
        summary: "A placa de vídeo gerencia a própria fila de trabalho. Necessário para o DLSS Frame Generation.",
        details: "Indicado para NVIDIA RTX 20 ou mais nova, AMD RX 5000 ou mais nova e Intel Arc. Em placas antigas ou vídeo integrado o ganho é pequeno; se notar travadas, desligue.",
        risk: Risk::Moderado,
        admin: true,
        reboot: true,
        recommended: false,
        kind: Kind::Registry(&[RegOp::new(
            Hklm,
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "HwSchMode",
            Dword(2),
            Dword(1),
        )]),
        post: Post::None,
    },
    Tweak {
        id: "power_plan",
        category: Category::Jogos,
        title: "Plano de energia Desempenho Máximo",
        summary: "Cria e ativa o plano \"Afina - Desempenho\": processador e dispositivos sem economia de energia.",
        details: "Ótimo para PCs de mesa. Em notebooks, use só na tomada: aumenta o calor e o consumo, e em notebooks só com vídeo integrado pode até reduzir o FPS (a CPU passa a disputar energia com o vídeo). Ao desligar, o plano anterior volta.",
        risk: Risk::Moderado,
        admin: true,
        reboot: false,
        recommended: false,
        kind: Kind::PowerPlan,
        post: Post::None,
    },
];
