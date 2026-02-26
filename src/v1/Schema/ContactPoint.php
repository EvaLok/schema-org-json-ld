<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ContactPoint extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ContactPoint';

	public function __construct(
		public null|string $telephone = null,
		public null|string $email = null,
		public null|string $contactType = null,
		public null|string $areaServed = null,
		public null|string $availableLanguage = null,
	) {}
}
