/*
 * Copyright (c) dodge-r / DodgeR; franziskusz.
 * This is a derivated and relicensed software under the GPL-3.0-only
 * The relicensing was done with generous support by Bromeon
 * If a copy of the GPL-3.0-only was not distributed with this file
 * You can obtain one at https://opensource.org/license/gpl-3-0
 *
 *
 * The dodge-r/dodge-the-creeps/rust/src code is derived
 * from the dodge-the-creeps example game by Bromeon and contributers
 * https://github.com/godot-rust/gdext/tree/master/examples/dodge-the-creeps
 * original License, MPL v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *
 * The "lift_weight()" function within dodge-r/dodge-the-creeps/rust/src/mob.rs lines 136-155
 * is derived from https://github.com/extrawurst/godot-rust-benchmark/blob/main/native-lib/src/bench.rs
 */

use godot::prelude::*;

mod hud;
mod main_scene;
mod mob;
mod player;
mod stats;

struct DodgeTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreeps {}
