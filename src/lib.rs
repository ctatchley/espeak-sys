//Copyright (C) 2015  Chandler Atchley

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
extern crate libc;

use libc::{c_int, c_char, c_uchar, c_short, size_t, c_uint, wchar_t, FILE};
use std::ffi::c_void;

pub const espeakRATE_MINIMUM: c_int = 80;
pub const espeakRATE_MAXIMUM: c_int = 450;
pub const espeakRATE_NORMAL:  c_int = 175;

pub const espeakINITIALIZE_PHONEME_EVENTS: c_int = 0x0001;
pub const espeakINITIALIZE_PHONEME_IPA:    c_int = 0x0002;
pub const espeakINITIALIZE_DONT_EXIT:      c_int = 0x8000;

pub const espeakCHARS_AUTO:  c_uint = 0;
pub const espeakCHARS_UTF8:  c_uint = 1;
pub const espeakCHARS_8BIT:  c_uint = 2;
pub const espeakCHARS_WCHAR: c_uint = 3;
pub const espeakCHARS_16BIT: c_uint = 4;

pub const espeakSSML: c_uint = 0x10;
pub const espeakPHONEMES: c_uint = 0x100;
pub const espeakENDPAUSE: c_uint = 0x1000;
pub const espeakKEEP_NAMEDATA: c_uint = 0x2000;

#[repr(C)]
pub enum espeak_EVENT_TYPE {
	espeakEVENT_LIST_TERMINATED = 0,
	espeakEVENT_WORD,
	espeakEVENT_SENTENCE,
	espeakEVENT_MARK,
	espeakEVENT_PLAY,
	espeakEVENT_END,
	espeakEVENT_MSG_TERMINATED,
	espeakEVENT_PHONEME,
	espeakEVENT_SAMPLERATE
}

#[repr(C)]
pub struct espeak_EVENT {
	pub event_type: espeak_EVENT_TYPE,
	pub text_position: c_int,
	pub length: c_int,
	pub audio_position: c_int,
	pub sample: c_int,
	pub user_data: *mut c_void,
	pub id: u64
}

#[repr(C)]
pub enum espeak_POSITION_TYPE {
	POS_CHARACTER = 1,
	POS_WORD,
	POS_SENTENCE
}

#[repr(C)]
pub enum espeak_AUDIO_OUTPUT {
	AUDIO_OUTPUT_PLAYBACK,
	AUDIO_OUTPUT_RETRIEVAL,
	AUDIO_OUTPUT_SYNCHRONOUS,
	AUDIO_OUTPUT_SYNCH_PLAYBACK
}

#[repr(C)]
pub enum espeak_ERROR {
	EE_OK = 0,
	EE_INTERNAL_ERROR = -1,
	EE_BUFFER_FULL = 1,
	EE_NOT_FOUND = 2
}

#[repr(C)]
pub enum espeak_PARAMETER {
	espeakSILENCE = 0,
	espeakRATE,
	espeakVOLUME,
	espeakPITCH,
	espeakRANGE,
	espeakPUNCTUATION,
	espeakCAPITALS,
	espeakWORDGAP,
	espeakOPTIONS,
	espeakINTONATION,
	espeakRESERVED1,
	espeakRESERVED2,
	espeakEMPHASIS,
	espeakVOICETYPE,
	N_SPEECH_PARAM
}

#[repr(C)]
pub enum espeak_PUNCT_TYPE {
	espeakPUNCT_NONE = 0,
	espeakPUNCT_ALL = 1,
	espeakPUNCT_SOME = 2
}

#[repr(C)]
pub struct espeak_VOICE {
	pub name: *const c_char,
	pub languages: *const c_char,
	pub identifier: *const c_char,
	pub gender: c_uchar,
	pub age: c_uchar,
	pub variant: c_uchar,
	xx1: c_uchar,
	score: c_int,
	spare: *mut c_void
}

impl espeak_VOICE {
    	pub fn new(
		name: *const c_char,
		languages: *const c_char,
		identifier: *const c_char,
		gender: c_uchar,
		age: c_uchar,
		variant: c_uchar) -> espeak_VOICE
	{
		espeak_VOICE {
			name,
			languages,
			identifier,
			gender,
			age,
			variant,
			xx1: 0,
			score: 0,
			spare: std::ptr::null_mut(),
		}
	}
}

pub type t_espeak_callback = extern "C" fn(*mut c_short, c_int, *mut espeak_EVENT) -> c_int;

#[link(name = "espeak-ng")]
extern "C" {
	pub fn espeak_Initialize(output: espeak_AUDIO_OUTPUT, buflength: c_int, path: *const c_char, options: c_int) -> c_int;
	pub fn espeak_SetSynthCallback(SynthCallback: t_espeak_callback);
	pub fn espeak_SetUriCallback(UriCallback: extern fn(c_int, *const c_char, *const c_char) -> c_int);
	pub fn espeak_Synth(text: *const c_void,
		size: size_t,
		position: c_uint,
		position_type: espeak_POSITION_TYPE,
		end_position: c_uint,
		flags: c_uint,
		unique_identifier: *mut c_uint,
		user_data: *mut c_void) -> espeak_ERROR;
	pub fn espeak_Synth_Mark(text: *const c_void,
		size: size_t,
		index_mark: *const c_char,
		end_position: c_uint,
		flags: c_uint,
		unique_identifier: *mut c_uint,
		user_data: *mut c_void) -> espeak_ERROR;
	pub fn espeak_Key(key_name: *const c_char) -> espeak_ERROR;
	pub fn espeak_Char(character: wchar_t) -> espeak_ERROR;
	pub fn espeak_SetParameter(parameter: espeak_PARAMETER, value: c_int, relative: c_int) -> espeak_ERROR;
	pub fn espeak_GetParameter(parameter: espeak_PARAMETER, current: c_int) -> c_int;
	pub fn espeak_SetPunctuationList(punctlist: *const wchar_t) -> espeak_ERROR;
	pub fn espeak_SetPhonemeTrace(value: c_int, stream: *mut FILE);
	pub fn espeak_TextToPhonemes(textptr: *const *const c_void, textmode: c_int, phonememode: c_int) -> *const c_char;
	pub fn espeak_CompileDictionary(path: *const c_char, log: *mut FILE, flags: c_int);
	pub fn espeak_ListVoices(voice_spec: *mut espeak_VOICE) -> *const *const espeak_VOICE;
	pub fn espeak_SetVoiceByName(name: *const c_char) -> espeak_ERROR;
	pub fn espeak_SetVoiceByProperties(voice_spec: *mut espeak_VOICE) -> espeak_ERROR;
	pub fn espeak_GetCurrentVoice() -> *mut espeak_VOICE;
	pub fn espeak_Cancel() -> espeak_ERROR;
	pub fn espeak_IsPlaying() -> c_int;
	pub fn espeak_Synchronize() -> espeak_ERROR;
	pub fn espeak_Terminate() -> espeak_ERROR;
	pub fn espeak_Info(path_data: *const *const c_char) -> *const c_char;
}
