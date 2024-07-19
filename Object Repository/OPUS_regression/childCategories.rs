<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>childCategories</name>
   <tag></tag>
   <elementGuidId>dc3a55ed-6e25-49c7-9f19-70ca42e2cfb8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;operationName\&quot;: \&quot;ChildCategories\&quot;,\n\t\&quot;variables\&quot;: {\n\t\t\&quot;storeId\&quot;: \&quot;opus\&quot;,\n\t\t\&quot;userId\&quot;: \&quot;{{userId}}\&quot;,\n\t\t\&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n\t\t\&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n\t\t\&quot;maxLevel\&quot;: 2,\n\t\t\&quot;onlyActive\&quot;: true,\n\t\t\&quot;productFilter\&quot;: \&quot;category.subtree:21057e9c-14df-48c1-be58-0c59a4f38f06\&quot;\n\t},\n\t\&quot;query\&quot;: \&quot;query ChildCategories($storeId: String!, $userId: String, $cultureName: String, $currencyCode: String, $categoryId: String, $maxLevel: Int, $onlyActive: Boolean, $productFilter: String) {\\n  childCategories(\\n    storeId: $storeId\\n    userId: $userId\\n    cultureName: $cultureName\\n    currencyCode: $currencyCode\\n    categoryId: $categoryId\\n    maxLevel: $maxLevel\\n    onlyActive: $onlyActive\\n    productFilter: $productFilter\\n  ) {\\n    __typename\\n    childCategories {\\n      id\\n      name\\n      level\\n      imgSrc\\n      priority\\n      slug\\n      parent {\\n        id\\n      }\\n      seoInfo {\\n        pageTitle\\n        metaKeywords\\n        metaDescription\\n      }\\n      breadcrumbs {\\n        title\\n        seoPath\\n      }\\n      childCategories {\\n        id\\n        name\\n        level\\n        imgSrc\\n        priority\\n        slug\\n        parent {\\n          id\\n        }\\n        seoInfo {\\n          pageTitle\\n          metaKeywords\\n          metaDescription\\n        }\\n        breadcrumbs {\\n          title\\n          seoPath\\n        }\\n      }\\n    }\\n  }\\n}\&quot;\n}&quot;,
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
      <name>Cookie</name>
      <type>Main</type>
      <value>${GlobalVariable.authorizationCookie}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/xapi/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'ca8ba2ca-1f4b-4a19-81c4-36a3591e2fc5'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ca185ac-47a1-421c-8585-7c56e74cb8ee</id>
      <masked>false</masked>
      <name>Cookie</name>
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
