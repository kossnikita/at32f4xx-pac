#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Field `HICKCAL` reader - Internal high-speed auto clock calibration"]
pub type HICKCAL_R = crate::FieldReader;
#[doc = "Field `HICKTRIM` reader - Internal high-speed auto clock trimming"]
pub type HICKTRIM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Internal high-speed auto clock calibration"]
    #[inline(always)]
    pub fn hickcal(&self) -> HICKCAL_R {
        HICKCAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Internal high-speed auto clock trimming"]
    #[inline(always)]
    pub fn hicktrim(&self) -> HICKTRIM_R {
        HICKTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("hickcal", &self.hickcal())
            .field("hicktrim", &self.hicktrim())
            .finish()
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`reset()` method sets CTRL2 to value 0x2080"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x2080;
}
