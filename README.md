# FDR Finder - A Freedomain Podcast Search Engine

An open-source search engine for [Freedomain Radio](https://www.freedomain.com/) podcasts. The site is live at https://fdr-finder.tommyvolk.com/.

## High-Level Design

### Meilisearch

When deciding what search engine to build on top of, the most obvious choice was [Elasticsearch](https://www.elastic.co/). However, there were a few downsides to this:
1. Since it's built on Java, it is fairly resource-hungry (specifically RAM) and the impact on hosting costs would be non-negligible
2. While it is very rich in functionality, this comes at the cost of developer complexity
3. Elasticsearch is built for scale, which isn't something we necessarily need since our dataset is ~5000 items (at time of writing) - this isn't so much a downside as it is a lack of upside

For these reasons, I decided to use [Meilisearch](https://www.meilisearch.com/). It's written in Rust, so it is extremely efficient and can easily run on a $5/month Digitalocean droplet. It is also very simple to setup, while still maintaining all of the features we want (see Meilisearch's [relevancy rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html)). The main downside of Meilisearch is that it lacks the horizontal scalability and complex feature set of Elasticsearch. However, these things aren't important for our use-case. And as a bonus, Meilisearch is designed to respond in <50ms, allowing for search-as-you-type functionality.

### Server-Side Caching

Since there are only ~5000 total podcasts (at time of writing), the server is easily able to hold all podcasts in memory, allowing for individual podcasts to be loaded without any database lookup.

## Contributing

Contributions are welcome! All successful pushes/merges into the master branch are automatically deployed to https://fdr-finder.tommyvolk.com/. Feel free to check out the issues page to see what needs to be done, and reach out to me in the issue comments or at tvolk131@gmail.com.

### Testing/Building/Running Locally

The repo contains shell scripts for building and running for local development. In the main repo folder, run ```sh build.sh``` or ```sh run.sh``` to build or run the code. The build script will run Webpack to create the JS client bundle, and then build the Rust server. The run command will execute the build script and then start the Rust server. Since these are shell scripts, they do not work on Windows. However, do not fear if you're using Windows like I am. Keep reading.

### Gitpod

I personally do all development for this project using [Gitpod](https://www.gitpod.io/). Gitpod provides ephemeral VS Code workspaces in the browser that are each connected to a cloud-based VM, giving you access to a consistent development environment with a Linux terminal regardless of what platform you're running on. In addition, this repo has a gitpod configuration file that prebuilds the code, allowing for near-instant dev environment setup with a pre-packed JS bundle and pre-compiled Rust server.
