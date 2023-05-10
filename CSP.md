# Chess Server Protocol

The "Chess Server Protocol" is used to communicate with a host that runs a game of chess. \
Two clients can connect at max identifying with an id they get by communicating with the host at the start of the game.

## Different types of client commands:

### Authorization:
Commands used for client-host communication. \
<br/>

#### join [code]:
Register user to the game with the code [code].

#### leave:
Unregister the user with the name [name] from the game. \
<br/>

### Configuration:
These commands are used to change settings or states inside the chess engine. \
<br/>

#### new [default|fen]:
This command is used to instantiate a new game. Both connected client
must send this command to perform the action. Default means a fresh game
with the standard formation, otherwise the field described by the FEN-string.

### Request:
These are commands to receive data from the ongoing game. \
<br/>

#### killed [color]:
Returns a list of killed [color] pieces. The response is a list of figure
symbols.

#### fen:
Returns the FEN-String of the current state of the ongoing game. \
<br/>

### Executive:
Commands to change the internal state of the ongoing game.

#### move [from] [to]:
Tries to move the piece standing on the [from] position to the [to]
position. The result is either 'ok' or 'error'

## Different types of server commands:

### General

General the server sends back Ok or Error. They both contain messages describing
the result or the error that occurred.

### Authorization:
Commands used for client-host communication. \
<br/>

#### join -> Ok [color] | Error [message]:
Return either the color you got assigned to or an error.

#### leave -> RO RESPONSE:
The server sends back no response.
<br/>

### Configuration:
These commands are used to change settings or states inside the chess engine. \
<br/>

#### new -> Ok [code] | Error [message]:
When the creation of a new game was a success the server sends back the code of the game.
Otherwise an error message

### Request:
These are commands to receive data from the ongoing game. \
<br/>

#### killed -> Ok [list] | Error [message]:
Returns the list of killed figures in the game if the user is associated
to a game. Otherwise, an error is sent back.

#### fen -> Ok [fen] | Error [message]:
Returns the FEN-String if the user is associated with a game. \
<br/>

### Executive:
Commands to change the internal state of the ongoing game.

#### move -> Ok | Error [message]:
Ok if the move was valid, otherwise an error.