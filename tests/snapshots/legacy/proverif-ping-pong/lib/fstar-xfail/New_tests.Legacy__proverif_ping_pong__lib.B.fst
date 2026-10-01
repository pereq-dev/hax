module New_tests.Legacy__proverif_ping_pong__lib.B
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_B0 = | B0 : t_B0

type t_B1 = { f_received:u8 }

type t_B1alt = | B1alt : t_B1alt

type t_B2 = | B2 : t_B2

let init_b (_: Prims.unit) : Core_models.Result.t_Result t_B0 Hax_lib_protocol.t_ProtocolError =
  Core_models.Result.Result_Ok (B0 <: t_B0)
  <:
  Core_models.Result.t_Result t_B0 Hax_lib_protocol.t_ProtocolError

let read_ping (e_state: t_B0) (msg: New_tests.Legacy__proverif_ping_pong__lib.t_Message)
    : Core_models.Result.t_Result t_B1 Hax_lib_protocol.t_ProtocolError =
  match msg <: New_tests.Legacy__proverif_ping_pong__lib.t_Message with
  | New_tests.Legacy__proverif_ping_pong__lib.Message_Ping received ->
    Core_models.Result.Result_Ok ({ f_received = received } <: t_B1)
    <:
    Core_models.Result.t_Result t_B1 Hax_lib_protocol.t_ProtocolError
  | New_tests.Legacy__proverif_ping_pong__lib.Message_Pong _ ->
    Core_models.Result.Result_Err
    (Hax_lib_protocol.ProtocolError_InvalidMessage <: Hax_lib_protocol.t_ProtocolError)
    <:
    Core_models.Result.t_Result t_B1 Hax_lib_protocol.t_ProtocolError

let read_ping_alt (e_state: t_B0) (msg: New_tests.Legacy__proverif_ping_pong__lib.t_Message)
    : Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError =
  match
    (match msg <: New_tests.Legacy__proverif_ping_pong__lib.t_Message with
      | New_tests.Legacy__proverif_ping_pong__lib.Message_Ping received ->
        (match received =. mk_u8 42 <: bool with
          | true ->
            Core_models.Option.Option_Some
            (Core_models.Result.Result_Ok (B1alt <: t_B1alt)
              <:
              Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError)
            <:
            Core_models.Option.t_Option
            (Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError)
          | _ ->
            Core_models.Option.Option_None
            <:
            Core_models.Option.t_Option
            (Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError))
      | _ ->
        Core_models.Option.Option_None
        <:
        Core_models.Option.t_Option
        (Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError))
    <:
    Core_models.Option.t_Option
    (Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError)
  with
  | Core_models.Option.Option_Some x -> x
  | Core_models.Option.Option_None  ->
    Core_models.Result.Result_Err
    (Hax_lib_protocol.ProtocolError_InvalidMessage <: Hax_lib_protocol.t_ProtocolError)
    <:
    Core_models.Result.t_Result t_B1alt Hax_lib_protocol.t_ProtocolError

let write_pong (state: t_B1)
    : Core_models.Result.t_Result (t_B2 & New_tests.Legacy__proverif_ping_pong__lib.t_Message)
      Hax_lib_protocol.t_ProtocolError =
  Core_models.Result.Result_Ok
  ((B2 <: t_B2),
    (New_tests.Legacy__proverif_ping_pong__lib.Message_Pong state.f_received
      <:
      New_tests.Legacy__proverif_ping_pong__lib.t_Message)
    <:
    (t_B2 & New_tests.Legacy__proverif_ping_pong__lib.t_Message))
  <:
  Core_models.Result.t_Result (t_B2 & New_tests.Legacy__proverif_ping_pong__lib.t_Message)
    Hax_lib_protocol.t_ProtocolError
