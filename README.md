# Docker-Cron

This program let's you schedule bash commands using a crontab file.

The use case is for docker container running tasks in a scheduled manner. Cron has some caveats that are undesirable in docker containers:

* The environment variables set in docker-compose or cli are not present
for cron jobs.
* PATH is not set as expected
* You might not want to deal with user permissions for each job
* You want to place the crontab files at a convenient location without
installing them


## Features

 * Crontab syntax
   * Respects comments starting with `#`
   * You can combine ranges and single entries 1,2-4,7 -> 1,2,3,4,7
   * You can use `/n` to only use the n-th entry in any range: `1-5/2` -> 
     2,4 or `*/5` -> 4,9,... (for ranges starting at 0 like minutes or hours)

 * Uses default environment (in contrast to cron)
 * Reports parsing errors at startup

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