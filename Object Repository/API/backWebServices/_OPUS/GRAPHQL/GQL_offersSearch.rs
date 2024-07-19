<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_offersSearch</name>
   <tag></tag>
   <elementGuidId>28f32f09-7b91-417f-9c99-659f5df2034e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;operationName\&quot;:\&quot;SearchProductsOffers\&quot;,\&quot;variables\&quot;:{\&quot;storeId\&quot;:\&quot;${storeId}\&quot;,\&quot;userId\&quot;:\&quot;${userId}\&quot;,\&quot;cultureName\&quot;:\&quot;en-US\&quot;,\&quot;currencyCode\&quot;:\&quot;USD\&quot;,\&quot;sort\&quot;:\&quot;\&quot;,\&quot;query\&quot;:\&quot;\&quot;,\&quot;filter\&quot;:\&quot;category.subtree:21057e9c-14df-48c1-be58-0c59a4f38f06 \\\&quot;SupplierOuterId\\\&quot;:\\\&quot;${supplierOuterId}\\\&quot;\&quot;,\&quot;first\&quot;:100,\&quot;after\&quot;:\&quot;0\&quot;},\&quot;query\&quot;:\&quot;query SearchProductsOffers($storeId: String!, $userId: String!, $currencyCode: String!, $cultureName: String, $filter: String, $after: String, $first: Int, $sort: String, $query: String, $fuzzy: Boolean, $fuzzyLevel: Int, $productIds: [String]) {\\n  products(\\n    storeId: $storeId\\n    userId: $userId\\n    after: $after\\n    first: $first\\n    filter: $filter\\n    sort: $sort\\n    currencyCode: $currencyCode\\n    cultureName: $cultureName\\n    query: $query\\n    fuzzy: $fuzzy\\n    fuzzyLevel: $fuzzyLevel\\n    productIds: $productIds\\n  ) {\\n    items {\\n      id\\n      availabilityData {\\n        isActive\\n        isAvailable\\n        isBuyable\\n        isInStock\\n        availableQuantity\\n      }\\n      price {\\n        actual {\\n          amount\\n          formattedAmount\\n          formattedAmountWithoutCurrency\\n          currency {\\n            symbol\\n          }\\n        }\\n        discountAmount {\\n          amount\\n          formattedAmount\\n        }\\n        sale {\\n          amount\\n          formattedAmount\\n        }\\n        list {\\n          amount\\n          formattedAmount\\n          formattedAmountWithoutCurrency\\n          currency {\\n            symbol\\n          }\\n        }\\n        discountPercent\\n      }\\n    }\\n  }\\n}\\n\&quot;}&quot;,
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
      <value>Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IkQ2Qzc4NzVBQzFGMUMzNkEyNjNEMDM1MUJFRDM3RDkyODc0MTc4QkYiLCJ4NXQiOiIxc2VIV3NIeHcyb21QUU5SdnROOWtvZEJlTDgiLCJ0eXAiOiJhdCtqd3QifQ.eyJuYW1lIjoiN2RkMjE5ZGYtZjJjMS00OTI0LWJiYzctZDM2YmFiMWMzNmNjIiwic3ViIjoiYWdlbmN5VXNlciIsImh0dHA6Ly9zY2hlbWFzLnhtbHNvYXAub3JnL3dzLzIwMDUvMDUvaWRlbnRpdHkvY2xhaW1zL2VtYWlsYWRkcmVzcyI6IjM0QGVtYWlsLmNvbSIsInJvbGUiOiJfX2N1c3RvbWVyIiwibWVtYmVySWQiOiI3NDIwMDY0OS1kMDNhLTQ1NTgtYmI2NC1kZDhhMzNjMTBiYmUiLCJvaV9hdV9pZCI6ImNlNTVhYTQwLTRmMWEtNDNiZi05ZDJhLTY1ZTkwNDRmZjA2ZiIsIm9pX3Rrbl9pZCI6ImZmNzkyMDFhLTg3ZWItNDVmYi04NTVlLWNlNzExNmE4OTczZSIsImF1ZCI6InJlc291cmNlX3NlcnZlciIsInNjb3BlIjoib2ZmbGluZV9hY2Nlc3MiLCJleHAiOjE2ODkxNjEwMzcsImlzcyI6Imh0dHBzOi8vb21uaWEtYXQucGFhcy5nb3ZpcnRvLmNvbS8iLCJpYXQiOjE2ODkxNTkyMzd9.JadGF6MtBZ6QPnD7HqwlgCAFnydshAT6ztdzP1rdh3iB9NMRTuZ83dY9ih_DmstmNQS3i4fQZOZfRWEwfY56LnTtdPCeL_o9mYuUNyUxzZJjIfYb3EhOHlUDnZ1PdOKFQeVMuWVsV8wdldWu9jB38wZ7dNH91Hi4OvGPsPCSNlzbVGRnFR13Yv2zaevXUI7niZS_J4PfI_nOXQ5tGO0_lSuVLD4BlDiIKXp59KAPfCK1xfr7QqXNj5bv_WAOdfJimvgxhaagesNH9ZBkOOFByzjMMSPcUP5mTv_1lqqAUu-SyiGhYmH4fL1wwvNjiNwO61yGtbnjWM8RUSKKePBj4DgEXB2CjvakaIY6dNVOPwvwiu-rTTdQUzpiW0VP_LVeyzOwCJ5k3CXMUjlFewKEbF3lsAijA1HeYFuZbif6-a56DC-lNciylLdZALyweH0ib2iCFFkluMZj25_sYsYrCxOTQ0jMazDGECHzq9uI9r3eT5Z6q_RCFzficafkN6sZX2yCRuEyhat5nvZ6RRNoMbmjHqVYQmYmlhqJNKMdyO13BQS8Kdhbtq4mfMrJFp4dorJ02wU6HoRuU_gPnKlxyXjq_MYtxatgLFmQlN11uMQaLN-waXAOAdNpUHfYwr5pAMTEubsZ948KdPxDBgWV9EEQeT4p1Zqdb9PpPONNMJM</value>
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
      <defaultValue>'7dd219df-f2c1-4924-bbc7-d36bab1c36cc'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'01'</defaultValue>
      <description></description>
      <id>1a9f3183-ea32-4229-b96f-731630382893</id>
      <masked>false</masked>
      <name>supplierOuterId</name>
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
