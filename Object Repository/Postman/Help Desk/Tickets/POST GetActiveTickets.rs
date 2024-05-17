<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST GetActiveTickets</name>
   <tag></tag>
   <elementGuidId>bd599544-632b-4ee5-b88f-4e507bea5a0b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;PageNumber\&quot;: \&quot;0\&quot;,\n\t\&quot;PageCount\&quot;: \&quot;25\&quot;,\n\t\&quot;state\&quot;: \&quot;2\&quot;,\n\t\&quot;filter\&quot;: {\n        \&quot;filters\&quot;: [\n            {\n                \&quot;field\&quot;: \&quot;statusName\&quot;,\n                \&quot;operator\&quot;: \&quot;eq\&quot;,\n                \&quot;value\&quot;: \&quot;Open\&quot;\n            },\n            {\n                \&quot;field\&quot;: \&quot;custom_6\&quot;,\n                \&quot;operator\&quot;: \&quot;contains\&quot;,\n                \&quot;value\&quot;: \&quot;asd\&quot;\n            },\n            {\n                \&quot;field\&quot;: \&quot;ticketSummary\&quot;,\n                \&quot;operator\&quot;: \&quot;contains\&quot;,\n                \&quot;value\&quot;: \&quot; a \&quot;\n            },\n            {\n                \&quot;field\&quot;: \&quot;createdDate\&quot;,\n                \&quot;operator\&quot;: \&quot;gte\&quot;,\n                \&quot;value\&quot;: \&quot;2019-07-17T06:00:00.000Z\&quot;\n            },\n            {\n                \&quot;field\&quot;: \&quot;createdDate\&quot;,\n                \&quot;operator\&quot;: \&quot;lt\&quot;,\n                \&quot;value\&quot;: \&quot;2019-07-18T06:00:00.000Z\&quot;\n            }\n        ],\n        \&quot;logic\&quot;: \&quot;and\&quot;\n    },\n    \&quot;sort\&quot;: null\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${HHD_API_Bearer}</value>
      <webElementGuid>c005b223-606c-46c8-9534-1bc97273b619</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>4166b2a9-cfeb-4dc8-ae9f-b029e210870d</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/Ticket/GetActiveTickets/3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Url</defaultValue>
      <description></description>
      <id>60ce9d8b-2fa6-4704-8fa4-e74a65c53edc</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>72c9ee7b-66cc-4f93-aed8-6e7580d020fa</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
