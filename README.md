# Docker-Cron: Simplified Task Scheduling in Docker

Docker-Cron is a streamlined tool designed to facilitate the scheduling of bash commands within Docker containers. Tailored for environments where traditional cron's complexities and limitations become apparent, Docker-Cron offers a simplified, more docker-centric approach to task automation.


build a stand-alone binary:
```bash
cargo build [--release]
```

run:
```bash
docker-cron <filename>
```

## Intended Use Case and Features


Ideal for Docker containers, Docker-Cron addresses specific challenges, including:

  * Ensuring environment variables set via Docker are accessible to scheduled tasks.
  
  * Simplifying the management of job permissions within the isolated Docker ecosystem.
  
  * Offering a direct and efficient method to manage and execute scheduled tasks without the overhead of traditional cron.

  * Supports crontab syntax with enhancements for ease of use in Docker environments.
    * Respects comments starting with `#`
    * You can combine ranges and single entries 1,2-4,7 -> 1,2,3,4,7
    * You can use `/n` to only use the n-th entry in any range: `1-5/2` ->  2,4 or `*/5` -> 4,9,... (for ranges starting at 0 like minutes or hours)
  
  * Direct execution of tasks leveraging the container's environment, bypassing common cron issues such as PATH discrepancies.
  * Error reporting at startup to ensure configuration integrity.

## Restrictions

 * Only one file file as argument is read as cli-argument
 * Environment variables cannot be specified in the crontab file
 * Crontab syntax errors are not tested extensively
 * Time is inefficiently checked every 60 s
    * Might lead to skipped minutes if the program is started really close to a minute switch
    * Process wakes up every minute unnecessarily
    * Feel free to contribute an improvement with dynamic sleep cycles 🙂

## Contributing

Open a ticket for found bugs and feature requests. (Also for the mentioned restrictions, if they bug you).
Or open a pull request for improvements