//! Category "Visual": desktop effects. Useful on PCs with integrated graphics.

use afina_platform::registry::Hive::Hkcu;

use crate::tweak::Val::{Dword, Sz};
use crate::tweak::{Category, Kind, Post, RegOp, Risk, Tweak};

pub(super) static TWEAKS: &[Tweak] = &[
    Tweak {
        id: "transparency_off",
        category: Category::Visual,
        title: "Desligar transparência",
        summary: "Menu Iniciar e barra de tarefas sem efeito de vidro: alivia vídeos integrados fracos.",
        details: "Mesma opção de Configurações > Personalização > Cores > Efeitos de transparência.",
        risk: Risk::Seguro,
        admin: false,
        reboot: false,
        recommended: false,
        kind: Kind::Registry(&[RegOp::new(
            Hkcu,
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "EnableTransparency",
            Dword(0),
            Dword(1),
        )]),
        post: Post::Theme,
    },
    Tweak {
        id: "animations_off",
        category: Category::Visual,
        title: "Desligar animações de janelas",
        summary: "Janelas abrem e minimizam na hora, sem animação.",
        details: "Vale totalmente depois de sair e entrar na conta do Windows.",
        risk: Risk::Seguro,
        admin: false,
        reboot: false,
        recommended: false,
        kind: Kind::Registry(&[
            RegOp::new(Hkcu, r"Control Panel\Desktop\WindowMetrics", "MinAnimate", Sz("0"), Sz("1")),
            RegOp::new(
                Hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "TaskbarAnimations",
                Dword(0),
                Dword(1),
            ),
        ]),
        post: Post::Theme,
    },
];
