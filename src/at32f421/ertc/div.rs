#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIVB` reader - Divider B"]
pub type DIVB_R = crate::FieldReader<u16>;
#[doc = "Field `DIVB` writer - Divider B"]
pub type DIVB_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `DIVA` reader - Divider A"]
pub type DIVA_R = crate::FieldReader;
#[doc = "Field `DIVA` writer - Divider A"]
pub type DIVA_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:14 - Divider B"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Divider A"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV")
            .field("diva", &self.diva())
            .field("divb", &self.divb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Divider B"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<DIV_SPEC> {
        DIVB_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Divider A"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<DIV_SPEC> {
        DIVA_W::new(self, 16)
    }
}
#[doc = "Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV to value 0x007f_00ff"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: u32 = 0x007f_00ff;
}
