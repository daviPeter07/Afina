//! Category "Sistema": Windows settings that free resources or fix specific problems.

use afina_platform::registry::Hive::{Hkcu, Hklm};

use crate::tweak::Val::{Delete, Dword};
use crate::tweak::{Category, Kind, Post, RegOp, Risk, Tweak};

pub(super) static TWEAKS: &[Tweak] = &[
    Tweak {
        id: "background_apps_off",
        category: Category::Sistema,
        title: "Bloquear apps da Microsoft Store em segundo plano",
        summary: "Apps da Store param de rodar escondidos, economizando memória e processador.",
        details: "Apps como WhatsApp ou Spotify da Store podem deixar de mostrar notificações com o app fechado. Programas tradicionais (.exe) não são afetados.",
        risk: Risk::Moderado,
        admin: false,
        reboot: false,
        recommended: false,
        kind: Kind::Registry(&[RegOp::new(
            Hkcu,
            r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
            "GlobalUserDisabled",
            Dword(1),
            Dword(0),
        )]),
        post: Post::None,
    },
    Tweak {
        id: "fast_startup_off",
        category: Category::Sistema,
        title: "Desligar Inicialização Rápida",
        summary: "\"Desligar\" passa a desligar de verdade e recarregar drivers a cada boot.",
        details: "Recomendado para quem usa dual boot com Linux ou tem problemas depois de atualizar drivers de vídeo. O boot fica alguns segundos mais lento.",
        risk: Risk::Seguro,
        admin: true,
        reboot: false,
        recommended: false,
        kind: Kind::Registry(&[RegOp::new(
            Hklm,
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
            "HiberbootEnabled",
            Dword(0),
            Dword(1),
        )]),
        post: Post::None,
    },
    Tweak {
        id: "rtc_utc",
        category: Category::Sistema,
        title: "Relógio da BIOS em UTC (dual boot com Linux)",
        summary: "Só para quem usa Linux no mesmo PC: impede que o relógio do Windows fique horas errado.",
        details: "O Linux grava a hora da BIOS em UTC e o Windows lê como horário local. Com o relógio errado, jogos online recusam o login (ex.: VAL 7 no VALORANT). NÃO ative se você não usa Linux.",
        risk: Risk::Moderado,
        admin: true,
        reboot: true,
        recommended: false,
        kind: Kind::Registry(&[RegOp::new(
            Hklm,
            r"SYSTEM\CurrentControlSet\Control\TimeZoneInformation",
            "RealTimeIsUniversal",
            Dword(1),
            Delete,
        )]),
        post: Post::None,
    },
    Tweak {
        id: "hvci_off",
        category: Category::Sistema,
        title: "Desligar Integridade de Memória (HVCI)",
        summary: "A própria Microsoft cita este recurso como causa de perda de FPS em jogos.",
        details: "Ganho típico de 5 a 10% em processadores mais antigos. Em troca, o Windows fica menos protegido contra drivers maliciosos. Alguns anti-cheats e programas corporativos exigem o recurso ligado: se algum reclamar, ligue de novo.",
        risk: Risk::Avancado,
        admin: true,
        reboot: true,
        recommended: false,
        kind: Kind::Registry(&[RegOp::new(
            Hklm,
            r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
            "Enabled",
            Dword(0),
            Dword(1),
        )
        .missing_means_on()]),
        post: Post::None,
    },
];
