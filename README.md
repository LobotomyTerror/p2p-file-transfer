# P2P File Transfer Project

This project is just a way to create an open source project using Rust. I am doing this to learn the language and get a feel for how it works in a web environment.

## Application Overview

Basically this application will be used for peer-to-peer file transmission, utilizing encryption and key API key generation to allow users to upload a file and transfer it via a key to another user to download said file. This is similar to [wormhole](https://wormhole.app/) but I wanted to try and implement something of my own as an open source project that others can build off of.

For the start I will keep it simple with using HTML/CSS and JavaScript for the frontend bits and then move to a framework as seen fit. For the backend this will involve an exploration into Rust, specifically [Rocket](https://rocket.rs/guide/v0.5/introduction/#introduction) as the REST API for the user interaction.

Beyond that there will be an implementation of a SQL database, so that API key links can be stored and accessed for users and their associated files that have been uploaded.

The idea is to keep to what wormhole does and only encrypt the files that are being uploaded and then generating an API key link for the users, and storing that information only. In no way will the data be saved into the database and will be automatically cleared out once it has passed a certain time limit.

More will come on this but the above is the starting point and general idea of this project
