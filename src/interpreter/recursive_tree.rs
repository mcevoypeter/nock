#![allow(unused_parens)]

use crate::interpreter::*;

/*==============================================================================
 * Nock operator trait implementations for Cell struct
 */

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        if let Noun::Atom(h) = *self.h {
            match h {
                Atom(0) => Err(Error {
                    msg: "/[0 a] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*self.t),
                Atom(2) => {
                    if let Noun::Cell(t) = *self.t {
                        Ok(*t.h)
                    } else {
                        Err(Error {
                            msg: "/[2 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                Atom(3) => {
                    if let Noun::Cell(t) = *self.t {
                        Ok(*t.t)
                    } else {
                        Err(Error {
                            msg: "/[3 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                Atom(n) => Cell {
                    h: Noun::Atom(Atom(2 + n % 2)).into_box(),
                    t: Cell {
                        h: Noun::Atom(Atom(n / 2)).into_box(),
                        t: self.t,
                    }
                    .fas()?
                    .into_box(),
                }
                .fas(),
            }
        } else {
            Err(Error {
                msg: "/[a b] cannot be evaluated when a is a cell".to_string(),
            })
        }
    }
}

impl Hax for Cell {
    fn hax(self) -> Result<Noun, Error> {
        if let (Noun::Atom(h), Noun::Cell(t)) = (*self.h, *self.t) {
            match h {
                Atom(0) => Err(Error {
                    msg: "#[0 a b] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*t.h),
                Atom(n) if 0 == n % 2 => Cell {
                    h: Noun::Atom(Atom(n / 2)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: t.h,
                            t: Cell {
                                h: Noun::Atom(Atom(n + 1)).into_box(),
                                t: t.t.clone(),
                            }
                            .fas()?
                            .into_box(),
                        })
                        .into_box(),
                        t: t.t,
                    })
                    .into_box(),
                }
                .hax(),
                Atom(n) => Cell {
                    h: Noun::Atom(Atom(n / 2)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Cell {
                                h: Noun::Atom(Atom(n - 1)).into_box(),
                                t: t.t.clone(),
                            }
                            .fas()?
                            .into_box(),
                            t: t.h,
                        })
                        .into_box(),
                        t: t.t,
                    })
                    .into_box(),
                }
                .hax(),
            }
        } else {
            Err(Error {
                msg: "#[a b] cannot be evaluated when a is cell and/or b is an atom".to_string(),
            })
        }
    }
}

impl Tar for Cell {
    fn tar(self) -> Result<Noun, Error> {
        if let Noun::Cell(t) = *self.t {
            match *t.h {
                Noun::Atom(Atom(0)) => Cell { h: t.t, t: self.h }.fas(),
                Noun::Atom(Atom(1)) => Ok(*t.t),
                Noun::Atom(Atom(2)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell {
                            h: Cell {
                                h: self.h.clone(),
                                t: tt.h,
                            }
                            .tar()?
                            .into_box(),
                            t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                        }
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(3)) => match (Cell { h: self.h, t: t.t }.tar()?) {
                    Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                    Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                },
                Noun::Atom(Atom(4)) => {
                    if let Noun::Atom(a) = (Cell { h: self.h, t: t.t }.tar()?) {
                        Ok(Noun::Atom(a.lus()))
                    } else {
                        Err(Error {
                            msg: "Cannot apply the + operator to a cell".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(5)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Ok(Noun::from_loobean(
                            Cell {
                                h: Cell {
                                    h: self.h.clone(),
                                    t: tt.h,
                                }
                                .tar()?
                                .into_box(),
                                t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                            }
                            .tis(),
                        ))
                    } else {
                        Err(Error {
                            msg: "*[a 5 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(6)) => {
                    if let Noun::Cell(tt) = *t.t {
                        if let Noun::Cell(ttt) = *tt.t {
                            Cell {
                                h: self.h.clone(),
                                t: Cell {
                                    h: Noun::Cell(Cell { h: ttt.h, t: ttt.t }).into_box(),
                                    t: Noun::Cell(Cell {
                                        h: Noun::Atom(Atom(0)).into_box(),
                                        t: Cell {
                                            h: Noun::Cell(Cell {
                                                h: Noun::Atom(Atom(2)).into_box(),
                                                t: Noun::Atom(Atom(3)).into_box(),
                                            })
                                            .into_box(),
                                            t: Noun::Cell(Cell {
                                                h: Noun::Atom(Atom(0)).into_box(),
                                                t: Cell {
                                                    h: self.h,
                                                    t: Noun::Cell(Cell {
                                                        h: Noun::Atom(Atom(4)).into_box(),
                                                        t: Noun::Cell(Cell {
                                                            h: Noun::Atom(Atom(4)).into_box(),
                                                            t: tt.h,
                                                        })
                                                        .into_box(),
                                                    })
                                                    .into_box(),
                                                }
                                                .tar()?
                                                .into_box(),
                                            })
                                            .into_box(),
                                        }
                                        .tar()?
                                        .into_box(),
                                    })
                                    .into_box(),
                                }
                                .tar()?
                                .into_box(),
                            }
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 6 b c] cannot be evaluated when c is an atom".to_string(),
                            })
                        }
                    } else {
                        Err(Error {
                            msg: "*[a 6 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(7)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell {
                            h: Cell { h: self.h, t: tt.h }.tar()?.into_box(),
                            t: tt.t,
                        }
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 7 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(8)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell {
                            h: Noun::Cell(Cell {
                                h: Cell {
                                    h: self.h.clone(),
                                    t: tt.h,
                                }
                                .tar()?
                                .into_box(),
                                t: self.h,
                            })
                            .into_box(),
                            t: tt.t,
                        }
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 8 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(9)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell {
                            h: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(2)).into_box(),
                                t: Noun::Cell(Cell {
                                    h: Noun::Cell(Cell {
                                        h: Noun::Atom(Atom(0)).into_box(),
                                        t: Noun::Atom(Atom(1)).into_box(),
                                    })
                                    .into_box(),
                                    t: Noun::Cell(Cell {
                                        h: Noun::Atom(Atom(0)).into_box(),
                                        t: tt.h,
                                    })
                                    .into_box(),
                                })
                                .into_box(),
                            })
                            .into_box(),
                        }
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 9 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(10)) => {
                    if let Noun::Cell(tt) = *t.t {
                        if let Noun::Cell(tth) = *tt.h {
                            Cell {
                                h: tth.h,
                                t: Noun::Cell(Cell {
                                    h: Cell {
                                        h: self.h.clone(),
                                        t: tth.t,
                                    }
                                    .tar()?
                                    .into_box(),
                                    t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                                })
                                .into_box(),
                            }
                            .hax()
                        } else {
                            Err(Error {
                                msg: "*[a 10 b c] cannot be evaluated when b is an atom"
                                    .to_string(),
                            })
                        }
                    } else {
                        Err(Error {
                            msg: "*[a 10 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(11)) => {
                    if let Noun::Cell(tt) = *t.t {
                        match *tt.h {
                            Noun::Atom(_) => Cell { h: self.h, t: tt.t }.tar(),
                            Noun::Cell(c) => Cell {
                                h: Noun::Cell(Cell {
                                    h: Cell {
                                        h: self.h.clone(),
                                        t: c.t,
                                    }
                                    .tar()?
                                    .into_box(),
                                    t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                                })
                                .into_box(),
                                t: Noun::Cell(Cell {
                                    h: Noun::Atom(Atom(0)).into_box(),
                                    t: Noun::Atom(Atom(3)).into_box(),
                                })
                                .into_box(),
                            }
                            .tar(),
                        }
                    } else {
                        Err(Error {
                            msg: "*[a 11 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(_)) => Err(Error {
                    msg: "unsupported opcode".to_string(),
                }),
                Noun::Cell(th) => Ok(Noun::Cell(Cell {
                    h: Cell {
                        h: self.h.clone(),
                        t: Noun::Cell(th).into_box(),
                    }
                    .tar()?
                    .into_box(),
                    t: Cell { h: self.h, t: t.t }.tar()?.into_box(),
                })),
            }
        } else {
            Err(Error {
                msg: "*[a b] cannot be evaluated when b is an atom".to_string(),
            })
        }
    }
}