# Docker-Cron

This program let's you schedule bash commands using a crontab file.

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