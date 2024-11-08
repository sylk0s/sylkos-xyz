use dioxus::prelude::*;
use crate::components::{
    document::Document,
    transtext::TransText,
};

#[component]
pub fn Random() -> Element {
    rsx! {
        Document {
            div {
                class: "text-rosewater p-4",

                h1 {
                    class: "text-4xl p-2",
                    "Random"
                }

                hr { class: "py-2" }

                h2 {
                    class: "text-2xl",
                    "Friends"
                }

                p {
                    class: "text-lg pb-4",
                    "Someday my friends will be cool enough to all have websites >:|"
                    br {}
                    Link {
                        to: "https://allalike.org",
                        TransText { text: "terminalvelocit<3" }
                    }
                    Link {
                        to: "https://adrs.pub/",
                        TransText { text: "adrs¯\\_(ツ)_/¯" }
                    }
                }

                hr { class: "py-2" }

                p {
                    class: "text-md m-0 leading-tight pb-4",
                    {include_str!("../../assets/berry.txt")}
                }

                hr { class: "py-2" }

                // h2 {
                //     class: "text-2xl",
                //     "Communities"
                // }

                // p {
                //     class: "text-lg",
                //     "I'm a part of a few communities, but I don't have any links to share yet"
                // }

                h2 {
                    class: "text-2xl",
                    "Random"
                }

                div {
                    class: "flex flex-row py-2",
                    Link {
                        class: "p-1",
                        to: "https://cyber.dabamos.de/88x31/index.html",
                        img {
                            class: "rounded-sm",
                            src: "img/88x31.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://cyber.dabamos.de/88x31/index.html",
                        img {
                            class: "rounded-sm",
                            src: "img/bu12.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://cyber.dabamos.de/88x31/index.html",
                        img {
                            class: "rounded-sm",
                            src: "img/kitten88.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://nixos.org",
                        img {
                            class: "rounded-sm",
                            src: "img/nixos.png",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://cyber.dabamos.de/88x31/index.html",
                        img {
                            class: "rounded-sm",
                            src: "img/nonbinary.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://cyber.dabamos.de/88x31/index.html",
                        img {
                            class: "rounded-sm",
                            src: "img/transnow2.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "neovim.io",
                        img {
                            class: "rounded-sm",
                            src: "img/vim.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://cyber.dabamos.de/88x31/index.html",
                        img {
                            class: "rounded-sm",
                            src: "img/wii.gif",
                        }
                    }
                }

                div {
                    class: "flex flex-row py-2",
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/amused.png",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/asdf.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/big_eyes.png",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/bzzzt.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/cattt.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/colon3.png",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "neovim.io",
                        img {
                            class: "rounded-sm",
                            src: "img/emoticons.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/genderhuh.png",
                        }
                    }
                }

                div {
                    class: "flex flex-row py-2",
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/isopods.png",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/nyan.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/possum.png",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://nixos.org",
                        img {
                            class: "rounded-sm",
                            src: "img/sparkl.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/strob.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/tetris.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/tnt.gif",
                        }
                    }
                    Link {
                        class: "p-1",
                        to: "https://collection.ju.mp/#",
                        img {
                            class: "rounded-sm",
                            src: "img/yippp.png",
                        }
                    }
                }
            }
        }
    }
}