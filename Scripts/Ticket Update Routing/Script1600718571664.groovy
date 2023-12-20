import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.maximizeWindow()

WebUI.click(findTestObject('Ticket Update Routing/a_Admin'))

WebUI.click(findTestObject('Ticket Update Routing/p_Ticket Routing'))

WebUI.click(findTestObject('Ticket Update Routing/a_Ticket Updates'))

WebUI.click(findTestObject('Ticket Update Routing/button_Create Rule'))

WebUI.setText(findTestObject('Ticket Update Routing/input__fieldName'), 'Ticket Update Rule')

WebUI.click(findTestObject('Ticket Update Routing/button_Add Condition'))

WebUI.selectOptionByValue(findTestObject('Ticket Update Routing/select_--- GetHelp Fields --- Assigned To D_b42230'), '9: 4-System', 
    true)

WebUI.selectOptionByValue(findTestObject('Ticket Update Routing/select_Is Is Not'), '1', true)

WebUI.selectOptionByValue(findTestObject('Ticket Update Routing/select_Approval Denied Approval Pending App_5199a0'), 'Approved', 
    true)

WebUI.selectOptionByValue(findTestObject('Ticket Update Routing/select_Unassigned------------------  Addy M_541135'), '11115', 
    true)

WebUI.selectOptionByValue(findTestObject('Ticket Update Routing/select_Open'), '1', true)

WebUI.click(findTestObject('Ticket Update Routing/button_Create'))

