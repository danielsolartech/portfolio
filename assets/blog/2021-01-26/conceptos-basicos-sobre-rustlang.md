---
lang: es
title: Conceptos básicos sobre Rustlang
description: Descripción de los conceptos básicos sobre el lenguaje de programación Rustlang, en conjunto a algunos proyectos que usan este maravilloso lenguaje y casos de uso reales con ejemplos de código.
image: /assets/images/projects/rustlang_es_icon.png
---

# ¿Qué es Rustlang?
Rustlang es un lenguaje de programación compilado; es decir, el código que escribas en el se pasa a lenguaje de máquina; este es multiparadigma y su propósito es ser un lenguaje seguro, concurrente y práctico. [Mozilla](https://www.mozilla.org/) comenzó a patrocinarlo en 2009 y para 2012 se lanzó el compilador de Rustlang escrito en Rustlang, llamado `rustc`.

Ahora bien, Rustlang es un lenguaje de esta época, pues su primera versión estable (La 1.0) fue lanzada el 15 de Mayo de 2015.

Rustlang hasta el momento es considerado "inmaduro", puesto que en sus primeras versiones cambiaba demasiado una respecto a la otra haciendo que fuese más difícil seguir una ruta de aprendizaje.

## Recolector de Basura (Garbage Collector)
Hablemos un poco sobre el recolector de basura de Rustlang, antes quiero aclarar que Rustlang no posee un recolector de basura, sino que se trata de un nuevo concepto llamado "[Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)"; este permite que sea mucho más rápido respecto a otros lenguajes como [Golang](https://golang.org/); sin embargo, esto no se nota hasta que se tenga una aplicación que requiera un enorme consumo de memoria.

# ¿Qué proyectos usan Rustlang?
A todo esto, ¿Qué proyectos usan Rustlang? [Discord](https://discord.com) es un gran ejemplo, esta empresa pasó de utilizar Go a utilizar Rust en algunas de sus características para mejorar la latencia entre el servidor y sus usuarios; esto fue explicado en un artículo publicado en su blog oficial ([Haz click aquí para leer el artículo](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f)). Otro gran ejemplo es la alianza entre Mozilla y Samsung para hacer el navegador web [Servo](https://github.com/servo/servo) paralelo a Firefox.

No podemos dejar atrás a Microsoft, puesto que este usa Rustlang en [Microsoft Azure IoT Edge](https://azure.microsoft.com/es-es/services/iot-edge/) (una plataforma usada para ejecutar servicios de Azure e inteligencia articial en dispositivos de IoT) para algunos componentes. También tenemos el ya conocido [Deno](https://deno.land/), el nuevo entorno de ejecución de JavaScript que usa el motor [V8 de Google](https://v8.dev/). Incluso, algunos contribuidores al kernel de Linux están evaluando las posibilidades de pasar todo el kernel a Rust o hacer los nuevos componentes en este tras ver su potencial en la comunidad y su gran aceptación.

# ¿Qué se puede hacer en Rustlang?
Con Rustlang podemos hacer infinidad de cosas desde un sistema operativo o una simple aplicación de consola; también aplicaciones de escritorio, videojuegos, páginas web o incluso utilizarlo en conjunto a JavaScript usando [WebAssembly](https://webassembly.org/).

# Recursos
- [Repositorio Oficial](https://github.com/rust-lang)
- [Algoritmos hechos con Rustlang](https://github.com/TheAlgorithms/Rust)
- [Tensorflow para Rustlang](https://github.com/tensorflow/rust)
- [Repositorios interesantes escritos en Rustlang](https://github.com/rust-unofficial/awesome-rust)
