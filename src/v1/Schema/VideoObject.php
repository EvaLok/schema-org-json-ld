<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class VideoObject extends TypedSchema {
public const A_SCHEMA_TYPE = 'VideoObject';

public function __construct(
public string $name,
/** @var string[] $thumbnailUrl */
public array $thumbnailUrl,
public string $uploadDate,
public null|string $description = null,
public null|string $contentUrl = null,
public null|string $embedUrl = null,
public null|string $duration = null,
public null|string $expires = null,
public null|string $regionsAllowed = null,
) {}
}
