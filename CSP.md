# Chess Server Protocol

The "Chess Server Protocol" is used to communicate with a host that runs a game of chess. \
Two clients can connect at max identifying with an id they get by communicating with the host at the start of the game.

## Different types of client commands:

### Authorization:
Commands used for client-host communication. \
<br/>

#### id [name]:
Register user with the name [name] to the game. Response is either 'ok'
or 'error'

#### leave [name]:
Unregister the user with the name [name] from the game. \
<br/>

### Configuration:
These commands are used to change settings or states inside the chess engine. \
<br/>

#### new [default|fen]:
This command is used to instantiate a new game. Both connected client
must send this command to perform the action. Default means a fresh game
with the standard formation, otherwise the field described by the FEN-string. 

#### cancelNew:
This command is used to withdraw the request for a new game. \
<br/>

### Request:
These are commands to receive data from the ongoing game. \
<br/>

#### moves [pos]:
Gets every possible move for the figure at the passed position.
The response is a list of positions( hv; h=horizontal, v=vertical ) seperated
by ';'.

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