# CORS middleware

To allow sites in the CORS middleware go to the project's root and create a `.env` file, then add the `WHITELIST` key.

For example:

```
// .env
WHITELIST=127.0.0.1
```

You can add multiple sites using commas between each site, for example:

```
WHITELIST=127.0.0.1,https://doc.rust-lang.org
```
