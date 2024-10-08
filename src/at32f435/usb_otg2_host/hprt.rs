#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HPRT_SPEC>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HPRT_SPEC>;
#[doc = "Field `PRTCONSTS` reader - Port connect status"]
pub type PRTCONSTS_R = crate::BitReader;
#[doc = "Field `PRTCONDET` reader - Port connect detected"]
pub type PRTCONDET_R = crate::BitReader;
#[doc = "Field `PRTCONDET` writer - Port connect detected"]
pub type PRTCONDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENA` reader - Port enable"]
pub type PRTENA_R = crate::BitReader;
#[doc = "Field `PRTENA` writer - Port enable"]
pub type PRTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENCHNG` reader - Port enable/disable change"]
pub type PRTENCHNG_R = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - Port enable/disable change"]
pub type PRTENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTOVRCACT` reader - Port overcurrent active"]
pub type PRTOVRCACT_R = crate::BitReader;
#[doc = "Field `PRTOVRCCHNG` reader - Port overcurrent change"]
pub type PRTOVRCCHNG_R = crate::BitReader;
#[doc = "Field `PRTOVRCCHNG` writer - Port overcurrent change"]
pub type PRTOVRCCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRES` reader - Port resume"]
pub type PRTRES_R = crate::BitReader;
#[doc = "Field `PRTRES` writer - Port resume"]
pub type PRTRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTSUSP` reader - Port suspend"]
pub type PRTSUSP_R = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - Port suspend"]
pub type PRTSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRST` reader - Port reset"]
pub type PRTRST_R = crate::BitReader;
#[doc = "Field `PRTRST` writer - Port reset"]
pub type PRTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNSTS` reader - Port line status"]
pub type PRTLNSTS_R = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - Port power"]
pub type PRTPWR_R = crate::BitReader;
#[doc = "Field `PRTPWR` writer - Port power"]
pub type PRTPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTTSTCTL` reader - Port test control"]
pub type PRTTSTCTL_R = crate::FieldReader;
#[doc = "Field `PRTTSTCTL` writer - Port test control"]
pub type PRTTSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTSPD` reader - Port speed"]
pub type PRTSPD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn prtconsts(&self) -> PRTCONSTS_R {
        PRTCONSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn prtcondet(&self) -> PRTCONDET_R {
        PRTCONDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port overcurrent active"]
    #[inline(always)]
    pub fn prtovrcact(&self) -> PRTOVRCACT_R {
        PRTOVRCACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn prtovrcchng(&self) -> PRTOVRCCHNG_R {
        PRTOVRCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("prtconsts", &self.prtconsts())
            .field("prtcondet", &self.prtcondet())
            .field("prtena", &self.prtena())
            .field("prtenchng", &self.prtenchng())
            .field("prtovrcact", &self.prtovrcact())
            .field("prtovrcchng", &self.prtovrcchng())
            .field("prtres", &self.prtres())
            .field("prtsusp", &self.prtsusp())
            .field("prtrst", &self.prtrst())
            .field("prtlnsts", &self.prtlnsts())
            .field("prtpwr", &self.prtpwr())
            .field("prttstctl", &self.prttstctl())
            .field("prtspd", &self.prtspd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    #[must_use]
    pub fn prtcondet(&mut self) -> PRTCONDET_W<HPRT_SPEC> {
        PRTCONDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    #[must_use]
    pub fn prtena(&mut self) -> PRTENA_W<HPRT_SPEC> {
        PRTENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    #[must_use]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W<HPRT_SPEC> {
        PRTENCHNG_W::new(self, 3)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    #[must_use]
    pub fn prtovrcchng(&mut self) -> PRTOVRCCHNG_W<HPRT_SPEC> {
        PRTOVRCCHNG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    #[must_use]
    pub fn prtres(&mut self) -> PRTRES_W<HPRT_SPEC> {
        PRTRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    #[must_use]
    pub fn prtsusp(&mut self) -> PRTSUSP_W<HPRT_SPEC> {
        PRTSUSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    #[must_use]
    pub fn prtrst(&mut self) -> PRTRST_W<HPRT_SPEC> {
        PRTRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    #[must_use]
    pub fn prtpwr(&mut self) -> PRTPWR_W<HPRT_SPEC> {
        PRTPWR_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    #[must_use]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W<HPRT_SPEC> {
        PRTTSTCTL_W::new(self, 13)
    }
}
#[doc = "OTGFS host port control and status register (OTGFS_HPRT)\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HPRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
