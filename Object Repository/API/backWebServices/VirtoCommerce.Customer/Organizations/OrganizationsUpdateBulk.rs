<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OrganizationsUpdateBulk</name>
   <tag></tag>
   <elementGuidId>7f3bd7ad-9d6f-4ef9-9422-c6f251968d39</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n  {\n    \&quot;name\&quot;: \&quot;${name1}\&quot;,\n    \&quot;description\&quot;: \&quot;Updated Org1\&quot;,\n    \&quot;id\&quot;: \&quot;${orgId1}\&quot;,\n    \&quot;groups\&quot;: [\n      \&quot;TEST UG1\&quot;,\n      \&quot;TEST UG2\&quot;\n    ],\n    \&quot;phones\&quot;: [\n      \&quot;0123456789\&quot;\n    ],\n    \&quot;emails\&quot;: [\n      \&quot;test@test.com\&quot;\n    ]\n  },\n  {\n    \&quot;name\&quot;: \&quot;${name2}\&quot;,\n    \&quot;description\&quot;: \&quot;Updated Org2\&quot;,\n    \&quot;id\&quot;: \&quot;${orgId2}\&quot;,\n    \&quot;groups\&quot;: [\n      \&quot;TEST UG1\&quot;,\n      \&quot;TEST UG2\&quot;\n    ],\n    \&quot;phones\&quot;: [\n      \&quot;0123456789\&quot;\n    ],\n    \&quot;emails\&quot;: [\n      \&quot;test@test.com\&quot;\n    ]\n  }\n]&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/organizations/bulk</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'f466f3cf-6d58-4653-8ed8-7d778efa8dd0'</defaultValue>
      <description>id of the org to update</description>
      <id>0a0aaca9-10df-4759-b815-cf9f13e8ed4c</id>
      <masked>false</masked>
      <name>orgId1</name>
   </variables>
   <variables>
      <defaultValue>'qewOrgUpdated1'</defaultValue>
      <description>organization name</description>
      <id>c153ac3b-33c7-4fa5-a299-858410602b4a</id>
      <masked>false</masked>
      <name>name1</name>
   </variables>
   <variables>
      <defaultValue>'qweOrgupdated2'</defaultValue>
      <description>organization name</description>
      <id>24f018da-0be4-473d-8ad5-d620fcbfff62</id>
      <masked>false</masked>
      <name>name2</name>
   </variables>
   <variables>
      <defaultValue>'0b7495f2-91c8-4d16-b534-acf999adfece'</defaultValue>
      <description>id of the org to update</description>
      <id>09044439-1181-4b19-bf86-27bfb40d4e7e</id>
      <masked>false</masked>
      <name>orgId2</name>
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
