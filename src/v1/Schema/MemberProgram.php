<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MemberProgram extends TypedSchema {
public const A_SCHEMA_TYPE = 'MemberProgram';

public function __construct(
public string $name,
public string $description,
/** @var MemberProgramTier[] $hasTiers */
public array $hasTiers,
public null|string $url = null,
) {}
}
