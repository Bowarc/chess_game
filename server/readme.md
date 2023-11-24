This is the server, it will host the conectiion between clients and the global logic

Quick question, do i make per client threads for the server too ?

I'll make some debug sht to check if thoses client takes a long time to operate (mostly de/serialising data)

About client management, i'll simply move clients arrounds, it's not a big deal.
edit: + they are now easily movable as they only store a couple of data and a channel to comunication with the proxy

Make a big room called lobby where everyone who connect gets put.
Any player in the lobby can create game by requesting an other player for a duel.
Matchmaking could be done with something similar, move a player into a struct MatchMaking{
    player: Vec<Player{id,client_handle,rating}>
}