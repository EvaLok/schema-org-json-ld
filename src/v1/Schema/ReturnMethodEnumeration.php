<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum ReturnMethodEnumeration: string {
	case ReturnAtKiosk = 'https://schema.org/ReturnAtKiosk';
	case ReturnByMail = 'https://schema.org/ReturnByMail';
	case ReturnInStore = 'https://schema.org/ReturnInStore';
}
