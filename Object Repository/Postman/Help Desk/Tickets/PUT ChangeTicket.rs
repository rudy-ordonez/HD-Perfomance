<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT ChangeTicket</name>
   <tag></tag>
   <elementGuidId>75b4a2c6-0074-4545-b752-475e37ffa46a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;TicketNumber\&quot;: 133050,\n\t\&quot;TicketSummary\&quot;: \&quot;Katalon generated ticket\&quot;,\n\t\&quot;TicketTypeID\&quot;: 0,\n\t\&quot;StatusID\&quot;: 5,\n\t\&quot;TicketPriorityID\&quot;: 3,\n\t\&quot;AssignedToUserID\&quot;: 0,\n\t\&quot;UpdatedByUserID\&quot;: 5,\n\t\&quot;TicketDetails\&quot;: {\n\t\t\&quot;TicketNumber\&quot;: 133050,\n\t\t\&quot;TicketDescription\&quot;: \&quot;Nothing to say. Something\&quot;,\n\t\t\&quot;ProblemTypeID\&quot;: 0,\n\t\t\&quot;Notes\&quot;: \&quot;\&quot;,\n\t\t\&quot;Resolved\&quot;: false,\n\t\t\&quot;ResolutionByUserID\&quot;: null\n\t},\n\t\&quot;TicketAssociatedTag\&quot;: [\n\t\t{\n\t\t\t\&quot;TicketAssociatedTagID\&quot;: 0,\n\t\t\t\&quot;TicketNumber\&quot;: 1011,\n\t\t\t\&quot;Tag\&quot;: \&quot;AATAG\&quot;,\n\t\t\t\&quot;Serial\&quot;: \&quot;aatag\&quot;,\n\t\t\t\&quot;SiteID\&quot;: \&quot;000\&quot;,\n\t\t\t\&quot;SiteName\&quot;: \&quot;000 - Holding\&quot;,\n\t\t\t\&quot;EntityName\&quot;: \&quot;Room: Empty\&quot;,\n\t\t\t\&quot;StatusDesc\&quot;: \&quot;Sold\&quot;,\n\t\t\t\&quot;ItemName\&quot;: \&quot;\\\&quot;B\\\&quot; Customizer\&quot;,\n\t\t\t\&quot;ItemTypeName\&quot;: \&quot;01\&quot;,\n\t\t\t\&quot;ManufacturerName\&quot;: \&quot;MAYNARD\&quot;,\n\t\t\t\&quot;ModelNumber\&quot;: \&quot;001\&quot;,\n\t\t\t\&quot;FundingSource\&quot;: \&quot;None\&quot;,\n\t\t\t\&quot;VendorName\&quot;: \&quot;\&quot;,\n\t\t\t\&quot;PurchasePrice\&quot;: 2,\n\t\t\t\&quot;UpdatedByUserID\&quot;: 5,\n\t\t\t\&quot;ResolutionAssociation\&quot;: false\n\t\t}\n\t],\n}&quot;,
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
      <webElementGuid>e1787f4c-6d7d-445f-9510-15d7b1b377f5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ba61f0bc-247f-4efd-b368-b2989cb66c63</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${HHD_API_Url}/api/Ticket/ChangeTicket</restUrl>
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
      <id>38fbfabb-e905-4ea5-87ef-caf1bc7c2121</id>
      <masked>false</masked>
      <name>HHD_API_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HHD_API_Bearer</defaultValue>
      <description></description>
      <id>e306f570-365e-4567-b5e8-910ae3fe8035</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
