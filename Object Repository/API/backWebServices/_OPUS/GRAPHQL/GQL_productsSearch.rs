<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_productsSearch</name>
   <tag></tag>
   <elementGuidId>7056d648-e60d-4841-a039-d499f0c5ab8a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;operationName\&quot;: \&quot;SearchProducts\&quot;,\n  \&quot;variables\&quot;: {\n    \&quot;storeId\&quot;: \&quot;opus\&quot;,\n    \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n    \&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n    \&quot;sort\&quot;: \&quot;\&quot;,\n    \&quot;withFacets\&quot;: true,\n    \&quot;withImages\&quot;: true,\n    \&quot;withProductOffers\&quot;: false,\n    \&quot;query\&quot;: \&quot;\&quot;,\n    \&quot;filter\&quot;: \&quot;category.subtree:21057e9c-14df-48c1-be58-0c59a4f38f06 \\\&quot;SupplierOuterId\\\&quot;:\\\&quot;01\\\&quot;\&quot;,\n    \&quot;first\&quot;: 100,\n    \&quot;after\&quot;: \&quot;0\&quot;\n  },\n  \&quot;query\&quot;: \&quot;query SearchProducts($storeId: String!, $userId: String!, $currencyCode: String!, $cultureName: String, $filter: String, $after: String, $first: Int, $sort: String, $query: String, $fuzzy: Boolean, $fuzzyLevel: Int, $productIds: [String], $withFacets: Boolean!, $withImages: Boolean!, $withProductOffers: Boolean!) {\\n  products(\\n    storeId: $storeId\\n    userId: $userId\\n    after: $after\\n    first: $first\\n    filter: $filter\\n    sort: $sort\\n    currencyCode: $currencyCode\\n    cultureName: $cultureName\\n    query: $query\\n    fuzzy: $fuzzy\\n    fuzzyLevel: $fuzzyLevel\\n    productIds: $productIds\\n  ) {\\n    totalCount\\n    items {\\n      name\\n      id\\n      code\\n      minQuantity\\n      maxQuantity\\n      inWishlist\\n      productType\\n      vendor {\\n        id\\n        name\\n        supplier {\\n          contractNumber\\n        }\\n      }\\n      hasVariations\\n      slug\\n      outline\\n      imgSrc\\n      images @include(if: $withImages) {\\n        url\\n        group\\n      }\\n      description(type: \\\&quot;QuickReview\\\&quot;) {\\n        content\\n      }\\n      availabilityData @include(if: $withProductOffers) {\\n        ...availabilityDataFields\\n      }\\n      price @include(if: $withProductOffers) {\\n        actual {\\n          ...moneyFields\\n        }\\n        discountAmount {\\n          amount\\n          formattedAmount\\n        }\\n        sale {\\n          amount\\n          formattedAmount\\n        }\\n        list {\\n          ...moneyFields\\n        }\\n        discountPercent\\n      }\\n      keyProperties {\\n        ...propertyFields\\n      }\\n    }\\n    term_facets @include(if: $withFacets) {\\n      name\\n      label\\n      terms {\\n        label\\n        term\\n        count\\n        isSelected\\n      }\\n    }\\n    range_facets @include(if: $withFacets) {\\n      name\\n      label\\n      ranges {\\n        label\\n        count\\n        from\\n        to\\n        includeFrom\\n        includeTo\\n        isSelected\\n      }\\n    }\\n  }\\n}\\n\\nfragment propertyFields on Property {\\n  name\\n  value\\n  type\\n  hidden\\n  valueType\\n  label\\n}\\n\\nfragment availabilityDataFields on AvailabilityData {\\n  isActive\\n  isAvailable\\n  isBuyable\\n  isInStock\\n  availableQuantity\\n  isEstimated\\n}\\n\\nfragment moneyFields on MoneyType {\\n  amount\\n  formattedAmount\\n  formattedAmountWithoutCurrency\\n  currency {\\n    ...currencyFields\\n  }\\n}\\n\\nfragment currencyFields on CurrencyType {\\n  code\\n  symbol\\n}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0141233-e5b2-4c87-ad29-33a320387433</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IkQ4OEZGRUFEMTk1OUMwREUyQUE4OTI2NEFBNEQyRTQ5N0I5MTgxMEEiLCJ4NXQiOiIySV8tclJsWndONHFxSkprcWswdVNYdVJnUW8iLCJ0eXAiOiJhdCtqd3QifQ.eyJuYW1lIjoiMDgyZjQzMzQtYjQxNC00Y2JjLThlNTEtNGI4NWU3NzFkNDA1Iiwic3ViIjoiYWdlbmN5VXNlciIsImh0dHA6Ly9zY2hlbWFzLnhtbHNvYXAub3JnL3dzLzIwMDUvMDUvaWRlbnRpdHkvY2xhaW1zL2VtYWlsYWRkcmVzcyI6IjU4QGVtYWlsLmNvbSIsInJvbGUiOiJfX2N1c3RvbWVyIiwibWVtYmVySWQiOiJlMTJhMThiOC02N2RhLTRhOWItOWFlMC1jOTBhM2JkYWZlODgiLCJvaV9hdV9pZCI6ImZmNzJkOWU1LTkwOTUtNDBhOC1hOTk3LTJkY2IzMTNlMWM1OCIsIm9pX3Rrbl9pZCI6IjdhYTQ0NzY4LTA5ZjMtNDZiNC04ZTRlLTQxZWQ0YjJlMGVlYyIsImF1ZCI6InJlc291cmNlX3NlcnZlciIsInNjb3BlIjoib2ZmbGluZV9hY2Nlc3MiLCJleHAiOjE2ODkwODc0MDgsImlzcyI6Imh0dHBzOi8vb21uaWEtYXQucGFhcy5nb3ZpcnRvLmNvbS8iLCJpYXQiOjE2ODkwODU2MDh9.l5bky3ZILaLNF8ds75gkV2Vv8vTdsF14Y3v8xURJh9HWj06PeVA5v3ZJyNzK71S0t9vE9_cWpeWugLBqSrrtOnJbvnMsywFHHcNavGf2PxBIQrmqaOJgixH07LPO4QnBzjGvVZFKYsT8Y8s_IGcjghXri6gaGyo2NW5xwlU7QsE7QpvyfRsgujssGSXysJAOXFuOXp_9Fknrby-WrdSNcJcpMRV1D6L5hcWtD_gZcALc6wjrWDEo6QSBzyweEa0SOycOEI6-kxQ5BQDKlTTj4L8WLTNA6JysoTo-Ej4ckT-yIuFUq41E1PaQ4KRdN-RiERQ3KfUMI3g_IA2lXYyRD6zgEmbdksNh4KHdB3Mf7udlxSCFWHVLTyFjex8cVAQHHQ2HBdpta4yEWokcYQZbF-SiBYX4ERyApVi2it21Iu4jOaeaBAsxmdB-cT7U9IrYYj7rpy1R065-ec-b_q55w3i3y1JhBzbmCaN01KI34-nu1UvzVeSSo8w1jPK36ys3jJ6fxHXkW8GdTpDH9hWuEE0jE23D3KbErPfkuzOwWym5awgVnaSPsEn0jrSaFV-xWtWrLKmmBWfKvjHcbwbCI9KyqcpSOpwMvMJEZNT3QiG8QXT4PPHKyRqcm-yOZUaejnp8e834E1NzotgU578F2ccGFFoYKLbjyyFU274AWK8</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'opus'</defaultValue>
      <description></description>
      <id>877818db-82bb-4530-836b-27ece9574709</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>'082f4334-b414-4cbc-8e51-4b85e771d405'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'01_100902'</defaultValue>
      <description></description>
      <id>36e065b6-d46f-4ab2-8e68-1e226a81fb01</id>
      <masked>false</masked>
      <name>productId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def prettyJson = JsonOutput.prettyPrint(response.getResponseBodyContent())
KeywordUtil.logInfo(prettyJson)
WS.verifyResponseStatusCode(response, 200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
