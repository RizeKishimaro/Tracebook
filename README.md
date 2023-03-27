<div align="center">

<img alt="GitHub release (by tag)" src="https://img.shields.io/github/downloads/RizeKishimaro/Tracebook/Alpha/total?color=red&label=V1.0-Alpha&logo=Github&logoColor=Github&style=flat-square">

<img alt="GitHub closed pull requests" src="https://img.shields.io/github/issues-pr-closed-raw/RizeKishimaro/Tracebook?style=flat-square">
<img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/RizeKishimaro/Tracebook?style=flat-square">
<img alt="GitHub contributors" src="https://img.shields.io/github/contributors/RizeKishimaro/Tracebook?style=flat-square">

# Tracebook

Just a fun project to practice our skill

# Installation

Both Tracebook's backend and desktop app can use in cross platform but I just compile it for x86_64 gnu linux.
If you wanna use in different archi or os just compile it from source code and please send me back binary file.
So I can add that bin file to Release.

<code> Compile From Source </code>

#### *Note Compiling code might take few minutes!

First please go to https://www.rust-lang.org to download and install rustup to compile pj! 

```
git clone https://github.com/RizeKishimaro/Tracebook 

cd Tracebook

cargo build --bin backend --release

cargo build --bin desktop --release
```

Compiled binary files should available in Tracebook/target/release folder!

<code> Arch based </code>

```
wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/backend-0.1.0-1-x86_64.pkg.tar.zst

wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/desktop-0.1.0-1-x86_64.pkg.tar.zst

sudo pacman -U backend-0.1.0-1-x86_64.pkg.tar.zst

sudo pacman -U desktop-0.1.0-1-x86_64.pkg.tar.zst
```
So now You can run it by backend and desktop

<code> Debian based </code>

```
wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/backend_0.1.0_amd64.deb

wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/desktop_0.1.0_amd64.deb

sudo dpkg -i backend_0.1.0_amd64.deb
sudo dpkg -i desktop_0.1.0_amd64.deb
```
Or Use You guys's fav gui installation, I use Arch BTW!

<code> Red Hat based </code>

```
wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/backend-0.1.0-1.x86_64.rpm

wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/desktop-0.1.0-1.x86_64.rpm

sudo rpm -i backend-0.1.0-1.x86_64.rpm

sudo rpm -i desktop-0.1.0-1.x86_64.rpm
```
I'm planing to make enterprise for you guys!

<code> Other gnu/linux_x86_64 </code>

```
wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/backend

wget https://github.com/RizeKishimaro/Tracebook/releases/download/Alpha/desktop
```
So you can run just ./blabla...

## Work Note

When You run backend, server will start in localhost:8090<br>
Than run desktop

## Preview

https://user-images.githubusercontent.com/85013114/227866106-6ee5f60c-515c-4a28-81d8-d47abf84be90.mp4

## Contributors
<a href="https://github.com/RizeKishimaro/Tracebook/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=RizeKishimaro/Tracebook" />
</a>
</div>
