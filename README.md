# Ozone

<p align="center">~ A webservice which scans files with ClamAV ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://crates.io/crates/ozone-ws" target="_blank">Crates.io</a>
</p>
<p align="center">Developed by <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (26/06/2023)</p>

<p align="center">
  <a href="https://opensource.org/license/mit/"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso-dev/ozone/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso-dev/ozone.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/ozone-ws"
    ><img
      src="https://img.shields.io/crates/d/ozone-ws.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/ozone-ws"
    ><img
      src="https://img.shields.io/crates/v/ozone-ws.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso-dev/ozone/actions"
    ><img
      src="https://github.com/veeso-dev/ozone/workflows/build-test/badge.svg"
      alt="Linux CI"
  /></a>
</p>

---

- [Ozone](#ozone)
  - [About Ozone](#about-ozone)
  - [Get started](#get-started)
    - [Run with docker](#run-with-docker)
  - [Ozone API](#ozone-api)
    - [Check](#check)
    - [Scan](#scan)
  - [Support the developer](#support-the-developer)
  - [Contributing and issues](#contributing-and-issues)
  - [Changelog](#changelog)
  - [License](#license)

---

## About Ozone

Ozone is a Rust web service which comes integrated with ClamAV. The service provides an API endpoint to scan files with ClamAV.

---

## Get started

### Run with docker

The entire ozone web service comes with a docker compose file to easily run the service on your machine.
Just run:

```sh
docker-compose build
docker-compose up -d
```

At this point ozone will be served on the specified port in the docker-compose file. (Default: `3010`)

## Ozone API

### Check

Check web service status:

```txt
GET /check
```

Response:

```json
{
  "status": "ok"
}
```

### Scan

Scan different files:

```txt
POST /scan
curl --request POST \
  --url http://localhost:3010/scan \
  --header 'Content-Type: multipart/form-data' \
  --form file=@/tmp/file1.txt \
  --form malware=@/tmp/eicarcom2.zip
```

Response:

```json
{
  "files": [
    {
      "name": "file",
      "filename": "file1.txt",
      "safe": true,
      "size": 222
    },
    {
      "name": "malware",
      "filename": "eicarcom2.zip",
      "safe": false,
      "size": 308,
      "threat": "Win.Test.EICAR_HDB-1"
    }
  ]
}
```

Where:

- `name`: is the filename
- `safe`: whether the file is safe
- `size`: the file size
- `threat`: the threat name found (optional; only if safe is `false`)

---

## Support the developer

If you like Ozone and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Contributing and issues

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve pavao, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog

View Ozone's changelog [HERE](CHANGELOG.md)

---

## License

Ozone is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
