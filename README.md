# load-balencer-rust


## Run
cargo run -- --load-balancer-ip <IP-LB:PORT> -b <IP-SRV1:PORT>, <IP-SRVX:PORT>, <IP-SRVXX:PORT> -p /health-check -i 3 -s 10 -r 5


## About

Conteneurs LXC utilisés pour ce projet.

LoadBalancer : contient le code de l'équilibreur de charge.
pour les upstreams, nous avons un template d'un serveur web actix LXC préconfiguré pour pouvoir déployer autant de serveurs web que nécessaire. Nous avons juste besoin de changer l'ip/port (nous avons gardé les chemins originaux que la doc nous a donné et nous avons ajouté le GET pour le health-check)


## Question

Avez-vous franchi toutes les étapes ? Jusqu'à l'étape 3 (+ un peu de 4)
Qu'avez-vous tenté ou non ?  Beaucoup de choses
Y a-t-il des bogues restants dont vous êtes au courant ? Parfois le load balancer redirege vers un serveur éteint
Qu'est-ce qui vous a plu dans cette mission ? Comprendre le fonctionnement d'un load balancer. 
Qu'est-ce qui vous a déplu ? La partie reflexion, synthaxe rust
Y a-t-il des parties sur lesquelles vous êtes restés bloqués ? Pas de blocage majeure, surtout des lenteur niveau reflexion 
Avez-vous essayé quelque chose pour améliorer les performances ? Code plus simple et moins de traitement "inutile"
Si vous deviez continuer à travailler sur ce projet, quelle serait la prochaine chose que vous mettriez en œuvre ? (Des fonctionnalités supplémentaires ? Un moyen d'améliorer les performances ?) Fichier .env et config plus modulaire.
Qu'est-ce qui vous a surpris ? Qu'avez-vous appris ? La gestion des connexions, la mise en place d'une architecture
