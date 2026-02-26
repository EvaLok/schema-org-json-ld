<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class FAQPage extends TypedSchema {
	public const A_SCHEMA_TYPE = 'FAQPage';

	public function __construct(
		/** @var Question[] $mainEntity */
		public array $mainEntity,
	) {}
}
