<?php

namespace EvaLok\SchemaOrgJsonLd\v1;

use PHPUnit\Event\Runtime\PHP;
use UnitEnum;

class JsonLdGenerator {
	public static function SchemaToJson(
		TypedSchema $schema
	): string {
		$obj = self::SchemaToObject($schema);

		return json_encode($obj, JSON_PRETTY_PRINT|JSON_UNESCAPED_SLASHES);
	}

	public static function SchemaToObject(
		TypedSchema $schema,
		$initialContext = true
	): array {
		$obj = [];

		if ( $initialContext ) {
			$obj['@context'] = "https://schema.org/";
			$initialContext = false;
		}

		$obj['@type'] = $schema::A_SCHEMA_TYPE;

		self::AddPropertiesToObject( get_object_vars($schema), $obj);

		return $obj;
	}

	private static function AddPropertiesToObject(
		Array $properties,
		Array &$obj,
	): void {

		foreach( $properties as $k => $v ){
			if ( is_string($v) || is_numeric($v) ) {
				$obj[$k] = $v;
			}

			else if ( $v instanceof UnitEnum ) {
				$obj[$k] = $v->value;
			}

			else if ( is_array($v) ) {
				if ( $v[0] instanceof TypedSchema ) {
					foreach( $v as $schema ) {
						$obj[$k][] = self::SchemaToObject(
							schema: $schema,
							initialContext: false,
						);
					}
				} else {
					foreach( $v as $element ) {
						if ( is_string($element) || is_numeric($element) ) {
							$obj[$k][] = $element;
						}

						else if ( $element instanceof UnitEnum ) {
							$obj[$k][] = $element->value;
						}
					}
				}
			}
		}
	}
}
