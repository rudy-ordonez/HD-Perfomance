<?xml version="1.0" encoding="UTF-8"?>
<TestSuiteEntity>
   <description></description>
   <name>AM Get Tags</name>
   <tag></tag>
   <isRerun>false</isRerun>
   <mailRecipient></mailRecipient>
   <numberOfRerun>0</numberOfRerun>
   <pageLoadTimeout>30</pageLoadTimeout>
   <pageLoadTimeoutDefault>true</pageLoadTimeoutDefault>
   <rerunFailedTestCasesOnly>false</rerunFailedTestCasesOnly>
   <rerunImmediately>false</rerunImmediately>
   <testSuiteGuid>0c1c654d-3d25-4571-b0f5-80d22b975556</testSuiteGuid>
   <testCaseLink>
      <guid>1436694a-26bc-4279-bdd2-125aae29f15d</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/AM/AM POST Authorize</testCaseId>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
   </testCaseLink>
   <testCaseLink>
      <guid>4ae27a21-0f86-41f0-b43c-130c454c16b8</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <iterationNameVariable>
         <defaultValue>GlobalVariable.TIPWebAPI_Tag</defaultValue>
         <description></description>
         <id>0bd5bb11-fa10-404d-8a78-fcb0a930c6a0</id>
         <masked>false</masked>
         <name>tag</name>
      </iterationNameVariable>
      <testCaseId>Test Cases/AM/GetTag</testCaseId>
      <testDataLink>
         <combinationType>ONE</combinationType>
         <id>4f8a30d0-28e8-4592-9703-aa3d0c485d6b</id>
         <iterationEntity>
            <iterationType>ALL</iterationType>
            <value></value>
         </iterationEntity>
         <testDataId>Data Files/AM QA Tags</testDataId>
      </testDataLink>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
      <variableLink>
         <testDataLinkId>4f8a30d0-28e8-4592-9703-aa3d0c485d6b</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>tag</value>
         <variableId>0bd5bb11-fa10-404d-8a78-fcb0a930c6a0</variableId>
      </variableLink>
   </testCaseLink>
</TestSuiteEntity>
