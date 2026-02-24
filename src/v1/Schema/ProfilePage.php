<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ProfilePage extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ProfilePage';

	public function __construct(
		public Person|Organization $mainEntity,
		public null|string $dateCreated = null,
		public null|string $dateModified = null,
	) {}
}
