/*
    References:
    https://konghq.com/blog/building-grpc-apis-with-rust

    Chose a savoir :
    En grpc, chaque machine est un client et un serveur.
    Le fichier proto défini l'ensemble des requêtes possibles et les réponses associées.
    Il faudrait implémenter 2 versions de UpSurge pour supporter grpc : UpSurgeClient & UpSurgeServer.

    Un fichier proto aurait cette forme :
    Le fichier proto permet de générer automatiquement les structures de données nécessaires à la communication.
    Les chiffres correspondent aux numéros des champs. Ils permettent de serialiser/deserialiser
    les messages envoyés/reçus par grpc très facilement.

    ######
    syntax = "proto3";
    package UpSurge;

    message CommandRequest {
        string address = 1;
        int32 port = 2;
        bool async = 3;
        string command = 4;
        repeated string args = 5;
    }

    message CommandResponse {
        bool success = 1;
        string output = 2;
    }
    ######

    Une fois le fichier proto écrit, il faut le compiler en rust.
    Pour cela, il faut installer les outils suivants :
        cargo install protobuf
        cargo install grpcio-compiler
        cargo install protobuf-codegen


    Ensuite, on peut compiler le fichier proto en rust :
        export PATH="$PATH:$HOME/.cargo/bin"
        export PROTOC_GEN_RUST="$(which protobuf-codegen)"
        protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` my_proto.proto



    Implémentation/Fonctionnement général :

    Evidemment, pour lancer une requete, il faut que le serveur distant soit en écoute.
    Autrement dit, il faut que UpSurge soit lancé sur la machine distante.

    Ainsi, le lancement d'un channel grpc doit passer par une phase d'auto-déploiement sur la machine distante.
    Pour se faire, on devrait se connecter normalement en SSH à cette machine, puis lancer UpSurge en mode client.
    Une fois UpSurge mis en route et la machine distante en écoute, on peut connecter la machine racine à la machine distante.
    Finalement, on pourra fermer la connexion SSH et n'utiliser que le channel grpc pour communiquer avec la machine distante.


    En bref, pour créer un channel grpc, il faut:
    - Se connecter en SSH à la machine distante
    - Lancer après (avoir télécharger si besoin) UpSurge, en version client.
    - On peut ensuite se connecter à la machine distante en grpc
    - Fermer la connexion SSH


*/
