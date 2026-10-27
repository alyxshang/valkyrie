# VALKYRIE

![Valkyrie CI](https://github.com/alyxshang/valkyrie/actions/workflows/rust.yml/badge.svg)
![Valkyrie Docker CI](https://github.com/alyxshang/valkyrie/actions/workflows/release.yml/badge.svg)

***A lightweight, decentralized, censorship-free social media platform.***

## ABOUT

This repository contains the source code for the backend for an API
server for an instance of a social media network running on the Valkyrie
protocol. The Valkyrie protocol is a protocol for running and using
decentralized social media where posting activities and follow activities are
broadcast and propagated across a network of multiple Valkyrie
instances. This implementation was made by me without the use of AI in
four days and the protocol was also designed by me. This API server is meant 
to be used with a custom frontend that anyone can write in whatever way and 
language they please.

## SPECIFICATION

A specification of the Valkyrie protocol can be found [here](https://alyxshang.boo/specs/valkyrie).

## USAGE 

The links below contain this implementation's documentation. Because
Valkyrie has two types of API endpoints, the API endpoints are
documented by their thematic group. There are two groups: endpoints for
client-to-server communication and enpoints for server-to-server
communication.

- [Client-to-Server API](docs/csapi.markdown)
- [Server-to-Server API](docs/ssapi.markdown)
- [Guide for administrators](docs/admins.markdown)

## CHANGELOG

### Version 0.1.0

- Initial release.
- Initial upload to GitHub.

## NOTE

- *Valkyrie* by *Alyx Shang*.
- Licensed under the [FSL v1](https://alyxshang.boo/content/fair-software-license).
