# PHP Framework Cache

This directory is the intended local cache for open-source PHP frameworks used by mEditor framework wizards, examples, indexing tests, and compiler compatibility checks.

## Download Status

On 2026-05-18, a network-restricted clone attempt was made for Laravel:

```bash
git clone --depth 1 https://github.com/laravel/framework.git third_party/php-frameworks/laravel-framework
```

The command failed because the environment could not connect to `github.com` over HTTPS. No framework source was downloaded into this directory during that attempt.

## Planned Framework Repositories

```bash
git clone --depth 1 https://github.com/laravel/framework.git third_party/php-frameworks/laravel-framework
git clone --depth 1 https://github.com/symfony/symfony.git third_party/php-frameworks/symfony
git clone --depth 1 https://github.com/codeigniter4/CodeIgniter4.git third_party/php-frameworks/codeigniter4
git clone --depth 1 https://github.com/cakephp/cakephp.git third_party/php-frameworks/cakephp
git clone --depth 1 https://github.com/yiisoft/yii2.git third_party/php-frameworks/yii2
git clone --depth 1 https://github.com/slimphp/Slim.git third_party/php-frameworks/slim
git clone --depth 1 https://github.com/laminas/laminas-mvc.git third_party/php-frameworks/laminas-mvc
git clone --depth 1 https://github.com/mezzio/mezzio.git third_party/php-frameworks/mezzio
git clone --depth 1 https://github.com/phalcon/cphalcon.git third_party/php-frameworks/phalcon
git clone --depth 1 https://github.com/spiral/framework.git third_party/php-frameworks/spiral
git clone --depth 1 https://github.com/nette/application.git third_party/php-frameworks/nette-application
```

## PHP Compiler Notes

- Standard PHP execution should use the open-source PHP interpreter from `php-src` with Composer and project-specific tooling.
- OPcache/JIT is part of modern PHP runtime configuration, but it is runtime optimization, not a universal standalone native compiler.
- PeachPie can compile compatible PHP projects to .NET.
- KPHP can compile supported PHP code through C++, but mEditor should run compatibility checks before presenting it as a build path for a framework project.
