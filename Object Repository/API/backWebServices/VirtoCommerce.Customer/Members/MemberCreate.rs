<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>MemberCreate</name>
   <tag></tag>
   <elementGuidId>e6755879-0ed0-41e6-9b7f-19359e10d401</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;${firstName}\&quot;,\n  \&quot;firstName\&quot;: \&quot;${firstName}\&quot;,\n  \&quot;middleName\&quot;: \&quot;${firstName}\&quot;,\n  \&quot;lastName\&quot;: \&quot;${lastName}\&quot;,\n  \&quot;fullName\&quot;: \&quot;${firstName}\&quot;,\n  \&quot;memberType\&quot;: \&quot;${memberType}\&quot;,\n  \&quot;isActive\&quot;: true,\n  \&quot;birthDate\&quot;: \&quot;2020-02-02T22:00:00.000Z\&quot;,\n  \&quot;timeZone\&quot;: \&quot;Europe/Moscow\&quot;,\n  \&quot;defaultLanguage\&quot;: \&quot;test lang\&quot;,\n  \&quot;photoUrl\&quot;: \&quot;photoUrl.test\&quot;,\n  \&quot;businessCategory\&quot;: \&quot;businessCategory\&quot;,\n  \&quot;description\&quot;: \&quot;Lorem ipsum\&quot;,\n  \&quot;groups\&quot;: [\n    \&quot;TEST UG1\&quot;,\n    \&quot;TEST UG2\&quot;\n  ],\n  \&quot;phones\&quot;: [\n    \&quot;0123456789\&quot;\n  ],\n  \&quot;emails\&quot;: [\n    \&quot;test@test.com\&quot;\n  ]\n}&quot;,
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
      <webElementGuid>2c012bc8-29f8-4e43-bd4a-c8e69eee8295</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>ec620fac-37bc-4288-9686-5a914f4cc29f</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/members</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Contact'</defaultValue>
      <description>type of the created member</description>
      <id>0ebcd8b2-e2ac-436f-9253-1ecbe7053e6e</id>
      <masked>false</masked>
      <name>memberType</name>
   </variables>
   <variables>
      <defaultValue>'qwe'</defaultValue>
      <description>name of the created member</description>
      <id>2908b59e-40e2-4b86-b07f-89df987e6b93</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'qwe'</defaultValue>
      <description>name of the created member</description>
      <id>d18ec4aa-c922-4353-8e4e-006d5af28cb2</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>'qwe'</defaultValue>
      <description>name of the created member</description>
      <id>0a68e940-74ca-479d-b77c-79e5bfacf6d9</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>'qwe'</defaultValue>
      <description>full name of the created member</description>
      <id>44e48c96-9ef5-4ea6-960f-6ff93e5f2a84</id>
      <masked>false</masked>
      <name>contactName</name>
   </variables>
   <variables>
      <defaultValue>' '</defaultValue>
      <description></description>
      <id>9f0b3ae9-671f-4ce4-b243-4bc71ce844c4</id>
      <masked>false</masked>
      <name>address</name>
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
WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
