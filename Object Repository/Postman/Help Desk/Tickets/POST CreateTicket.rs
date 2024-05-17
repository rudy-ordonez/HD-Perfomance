<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST CreateTicket</name>
   <tag></tag>
   <elementGuidId>8b98740f-41df-45e9-ad92-492ad1628acb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;TicketSummary\&quot;: \&quot;Katalon generated ticket\&quot;,\n    \&quot;TicketTypeID\&quot;: \&quot;0\&quot;,\n    \&quot;StatusID\&quot;: \&quot;1\&quot;,\n    \&quot;TicketPriorityID\&quot;: \&quot;3\&quot;,\n    \&quot;AssignedToUserID\&quot;: \&quot;11\&quot;,\n    \&quot;CreatedByUserID\&quot;: \&quot;3\&quot;,\n    \n    \&quot;TicketDetails\&quot;: {\n        \&quot;TicketDescription\&quot;: \&quot;The Theory of Relativity, proposed by the Jewish physicist Albert Einstein (1879-1955) in the early part of the 20th century, is one of the most significant scientific advances of our time. Although the concept of relativity was not introduced by Einstein, his major contribution was the recognition that the speed of light in a vacuum is constant and an absolute physical boundary for motion. This does not have a major impact on a person\u0027s day-to-day life since we travel at speeds much slower than light speed. For objects travelling near light speed, however, the theory of relativity states that objects will move slower and shorten in length from the point of view of an observer on Earth. Einstein also derived the famous equation, E \u003d mc2, which reveals the equivalence of mass and energy.\&quot;,\n        \&quot;ProblemTypeID\&quot;: \&quot;1\&quot;,\n        \&quot;Notes\&quot;: \&quot;General testing note\&quot;,\n        \&quot;Resolved\&quot;: false,\n        \&quot;Resolution\&quot;: \&quot;\&quot;,\n        \&quot;ResolutionDate\&quot;: null,\n        \&quot;ResolutionBy\&quot;: \&quot;0\&quot;,\n        \&quot;CreatedByEmail\&quot;: \&quot;false\&quot;,\n        \&quot;TicketSourceID\&quot;: 1\n    },\n    \&quot;TicketAssociatedTag\&quot;: [\n        {\n \t\&quot;inventoryID\&quot;: \&quot;\&quot;,\n    \&quot;tag\&quot;: \&quot;\&quot;,\n    \&quot;serial\&quot;: \&quot;\&quot;,\n    \&quot;siteUid\&quot;: \&quot;\&quot;,\n    \&quot;siteID\&quot;: \&quot;\&quot;,\n    \&quot;siteName\&quot;: \&quot;\&quot;,\n    \&quot;entityID\&quot;: \&quot;\&quot;,\n    \&quot;entityUid\&quot;: null,\n    \&quot;entityTypeID\&quot;: \&quot;\&quot;,\n    \&quot;entityName\&quot;: \&quot;\&quot;,\n    \&quot;statusID\&quot;: \&quot;\&quot;,\n    \&quot;statusDesc\&quot;: \&quot;\&quot;,\n    \&quot;itemID\&quot;: \&quot;\&quot;,\n    \&quot;itemName\&quot;: \&quot;\&quot;,\n    \&quot;itemTypeID\&quot;: \&quot;\&quot;,\n    \&quot;itemTypeName\&quot;: \&quot;\&quot;,\n    \&quot;itemTypeImageUrl\&quot;: \&quot;\&quot;,\n    \&quot;manufacturerID\&quot;: \&quot;\&quot;,\n    \&quot;manufacturerName\&quot;: \&quot;\&quot;,\n    \&quot;modelNumber\&quot;: \&quot;\\\&quot;\&quot;,\n    \&quot;fundingSourceID\&quot;: \&quot;\&quot;,\n    \&quot;fundingSource\&quot;: \&quot;\&quot;,\n    \&quot;vendorID\&quot;: \&quot;\&quot;,\n    \&quot;vendorName\&quot;: \&quot;\&quot;,\n    \&quot;purchasePrice\&quot;: \&quot;\&quot;,\n    \&quot;archiveID\&quot;: 0,\n    \&quot;archiveDate\&quot;: null,\n    \&quot;createdByUserID\&quot;: 0,\n    \&quot;resolutionAssociation\&quot;: null,\n    \&quot;roomForTicket\&quot;: \&quot;\&quot;\n        }\n  \t]\n}&quot;,
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
      <webElementGuid>082e3dd0-70b3-4d7c-bccb-00d2c0bb5b9b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1ea08cf7-0877-4eb9-8600-869296440f07</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/Ticket/CreateTicket</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Url</defaultValue>
      <description></description>
      <id>8b066a32-c478-4ab6-acab-db9cdec9c7e1</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>e0ba8ef4-76f8-4d00-9f8e-b8543f1efb62</id>
      <masked>false</masked>
      <name>HHD_API_Bearer</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Tag</defaultValue>
      <description></description>
      <id>2f5f0f44-fc80-40fd-85f0-33eb298d4101</id>
      <masked>false</masked>
      <name>HHD_API_Tag</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
