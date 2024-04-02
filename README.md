# Docker-Cron

This program let's you schedule bash commands using a crontab file.

## Features

 * Crontab syntax
   * Respects comments starting with `#`
   * You can combine ranges and single entries 1,2-4,7 -> 1,2,3,4,7

 * Uses default environment (in contrast to cron)
 * Reports parsing errors at startup

## Restrictions

 * Only one file file as argument is read as cli-argument
 * Environment variable cannot be specified in the crontab file
 * Crontab syntax errors are not tested extensively
 * Time is inefficiently checked every 60 s
    * Might lead to skipped minutes if the program is started close to a minute switch
    * Process wakes up every minute unnecessarily
    * Feel free to contribute a improvement with dynamic sleep cycles 🙂

## Contributing

Open a ticket for found bugs and feature requests. (Also for the mentioned restrictions, if they bug you).
Or open a pull request for improvements