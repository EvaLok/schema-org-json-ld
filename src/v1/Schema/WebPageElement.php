<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class WebPageElement extends TypedSchema {
	public const A_SCHEMA_TYPE = 'WebPageElement';

	public function __construct(
		public bool $isAccessibleForFree,
		public string $cssSelector,
	) {}
}
