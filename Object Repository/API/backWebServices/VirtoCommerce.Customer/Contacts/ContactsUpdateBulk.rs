<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ContactsUpdateBulk</name>
   <tag></tag>
   <elementGuidId>7596d14d-81af-4ede-a588-d749babd19ea</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n{\n\n  \&quot;firstName\&quot;: \&quot;JohnFirst X\&quot;,\n  \&quot;lastName\&quot;: \&quot;DoeLast Y\&quot;,\n  \&quot;fullName\&quot;: \&quot;${fullName1}\&quot;,\n\n  \&quot;id\&quot;: \&quot;${contactId1}\&quot;,\n  \&quot;addresses\&quot;: [\n    {\n    \t\&quot;addressType\&quot;: \&quot;Billing\&quot;,\n\t    \&quot;firstName\&quot;: \&quot;JohnFirst\&quot;,\n\t\t\&quot;lastName\&quot;: \&quot;DoeLast\&quot;,\n        \&quot;countryCode\&quot;: \&quot;USA\&quot;,    \n        \&quot;countryName\&quot;: \&quot;United States\&quot;,\n        \&quot;regionName\&quot;: \&quot;test region\&quot;,\n        \&quot;city\&quot;: \&quot;Test city\&quot;,\n        \&quot;line1\&quot;: \&quot;Test adress line 1\&quot;,\n        \&quot;line2\&quot;: \&quot;Test adress line 2\&quot;,\n        \&quot;postalCode\&quot;: \&quot;X34656703\&quot;,\n        \&quot;name\&quot;: \&quot;Qwest\&quot;\n    }\n  ],\n  \&quot;groups\&quot;: [\n    \&quot;TEST UG1\&quot;,\n    \&quot;TEST UG2\&quot;\n  ],\n  \&quot;phones\&quot;: [\n    \&quot;00000000\&quot;\n  ],\n  \&quot;emails\&quot;: [\n    \&quot;test@test.com\&quot;\n  ]\n},\n {\n\n  \&quot;firstName\&quot;: \&quot;JohnFirst X\&quot;,\n  \&quot;lastName\&quot;: \&quot;DoeLast Y\&quot;,\n  \&quot;fullName\&quot;: \&quot;${fullName2}\&quot;,\n\n  \&quot;id\&quot;: \&quot;${contactId2}\&quot;,\n  \&quot;addresses\&quot;: [\n    {\n    \t\&quot;addressType\&quot;: \&quot;Billing\&quot;,\n\t    \&quot;firstName\&quot;: \&quot;JohnFirst\&quot;,\n\t\t\&quot;lastName\&quot;: \&quot;DoeLast\&quot;,\n        \&quot;countryCode\&quot;: \&quot;USA\&quot;,    \n        \&quot;countryName\&quot;: \&quot;United States\&quot;,\n        \&quot;regionName\&quot;: \&quot;test region\&quot;,\n        \&quot;city\&quot;: \&quot;Test city\&quot;,\n        \&quot;line1\&quot;: \&quot;Test adress line 1\&quot;,\n        \&quot;line2\&quot;: \&quot;Test adress line 2\&quot;,\n        \&quot;postalCode\&quot;: \&quot;X34656703\&quot;,\n        \&quot;name\&quot;: \&quot;Qwest\&quot;\n    }\n  ],\n  \&quot;groups\&quot;: [\n    \&quot;TEST UG1\&quot;,\n    \&quot;TEST UG2\&quot;\n  ],\n  \&quot;phones\&quot;: [\n    \&quot;00000000\&quot;\n  ],\n  \&quot;emails\&quot;: [\n    \&quot;test@test.com\&quot;\n  ]\n} \n]&quot;,
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
      <webElementGuid>eb4d3b0f-2001-4d5e-8dd8-b6e1962e4e2d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>3d1bcacd-41e2-452d-9b65-72f94bbd9de6</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/contacts/bulk</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Qwe ContactUpdated'</defaultValue>
      <description>name of bulk created contact</description>
      <id>b45ced9c-f763-4af5-8cdf-d7b7f4da6522</id>
      <masked>false</masked>
      <name>fullName1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description>id of bulk created contact</description>
      <id>0a9326b4-2b4b-4b84-b8dd-c34f9662c237</id>
      <masked>false</masked>
      <name>contactId1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>85a82070-c5d9-4f69-bf6e-beefd7ee53ea</id>
      <masked>false</masked>
      <name>fullName2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>22b31efa-f03a-47d5-a1a8-e33f08a44c65</id>
      <masked>false</masked>
      <name>contactId2</name>
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
WS.verifyResponseStatusCode(response, 204)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
