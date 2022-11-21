<p align="center">
	<picture>
		<source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/tsanga/orbt/master/docs/_assets/logo-long-w.png" width="300">
		<img src="https://raw.githubusercontent.com/tsanga/orbt/master/docs/_assets/logo-long-b.png" width="300">
	</picture>
</p>

<div align="center">

[![License][mit-badge]][mit-url]
[![CI: web][ci-web-badge]][ci-web-url]
[![api][ci-api-badge]][ci-api-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-indigo.svg
[mit-url]: LICENSE
[ci-web-badge]: https://github.com/tsanga/orbt/actions/workflows/web.yml/badge.svg
[ci-web-url]: https://github.com/tsanga/orbt/actions/workflows/web.yml
[ci-api-badge]: https://github.com/tsanga/orbt/actions/workflows/api.yml/badge.svg
[ci-api-url]: https://github.com/tsanga/orbt/actions/workflows/api.yml

</div>

> ⚠️ **orbt** is still being built and is in very early stages -- **nothing really works yet**.

**orbt** is a collection of programs that work together to allow you to control a _virtual machine_<sub>1</sub> through your browser, enabling you to browse the internet and consume media with friends. It's kind of like Prime Video's Watch Party or DisneyPlus' GroupWatch, but you can visit **any** website.

**orbt** is composed of four programs/services:

- [orbt/api](../api):
  - graphql api written in rust
  - handles user data<sub>2</sub>, sessions, etc
  - handles server->client state updates via ws (gql subscriptions), i.e: chat, etc
- [orbt/web](../web):
  - web frontend, typescript, built in next.js 13 (appDir beta<sub>3</sub>, React 18 ✅)
  - [Check out the Figma](https://www.figma.com/file/nUauBElRMVAg2BTo88rS3v/orbt.tv-design?node-id=183%3A1387&t=TAx1P52458OxiLqN-1)
- [orbt/cli](../cli):
  - "gateway" to running api, web and vm together
  - interactive instance creation process, setting room name, password, etc
  - packages web and api into a single executable
- ~~[orbt/vm](../vm)~~:
  - boots up virtual display, browser, audio supporting software
  - captures virtual display via ffmpeg to relay over WebRTC
  - process kbm actions over WS connection from remote user in control

<sub> 1 _it's not actually a virtual machine, but a light-weight container running xvfb, chromium, ffmpeg, [orbt/vm](../vm) and some other utils_ </sub>  
<sub> 2 _user data is short lived, tied to a single temporary orbt instance, nothing persists. this may change in the future, but the goal of the project right now is for it to be something that is self-hosted, spun up on demand, not requiring users to register or login, etc_</sub>  
<sub> 3 we're testing out next.js' [appDir beta](https://beta.nextjs.org/docs/getting-started), so a lot of the std FE ecosystem tools aren't compatible yet or require patching, we may find ourselves blocked by something upstream.</sub>
